//! WiFi Telemetry System - Modular Version
#![no_std]
#![no_main]

use cyw43::JoinOptions;
use cyw43_pio::{PioSpi, DEFAULT_CLOCK_DIVIDER};
use defmt::*;
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_net::{Config, Stack, StackResources};
use embassy_rp::bind_interrupts;
use embassy_rp::clocks::RoscRng;
use embassy_rp::gpio::{Level, Output};
use embassy_rp::init;
use embassy_rp::peripherals::*;
use embassy_rp::pio::{InterruptHandler, Pio};
use embassy_time::{Duration, Timer};
use panic_probe as _;
use rand_core::RngCore;
use static_cell::StaticCell;

// Import our modules (finally using them!)
mod config;
mod drivers;
mod error;
mod network;
mod tasks;

use config::{TelemetryConfig, WiFiConfig};
use drivers::{Led, WiFiDriver};
use network::NetworkStack;
use tasks::{
    blinker_task, cyw43_task, network_task, telemetry_with_retry_task, TelemetryTaskConfig,
};

// WiFi credentials from build-time environment variables
const WIFI_NETWORK: &str = env!("WIFI_NETWORK");
const WIFI_PASSWORD: &str = env!("WIFI_PASSWORD");

/// LED Debug Patterns
async fn led_startup_pattern(led: &mut Led<'_>) {
    for _ in 0..10 {
        led.set_high();
        Timer::after(Duration::from_millis(100)).await;
        led.set_low();
        Timer::after(Duration::from_millis(100)).await;
    }
}

async fn led_wifi_connecting_pattern(led: &mut Led<'_>) {
    for _ in 0..10 {
        led.set_high();
        Timer::after(Duration::from_millis(300)).await;
        led.set_low();
        Timer::after(Duration::from_millis(300)).await;
    }
}

async fn led_success_pattern(led: &mut Led<'_>) {
    for _ in 0..5 {
        led.set_high();
        Timer::after(Duration::from_millis(100)).await;
        led.set_low();
        Timer::after(Duration::from_millis(100)).await;
    }
}

async fn led_error_pattern(led: &mut Led<'_>) -> ! {
    loop {
        for _ in 0..3 {
            led.set_high();
            Timer::after(Duration::from_millis(200)).await;
            led.set_low();
            Timer::after(Duration::from_millis(200)).await;
        }
        Timer::after(Duration::from_secs(2)).await;
    }
}

static STACK: StaticCell<Stack<'static>> = StaticCell::new();

#[embassy_executor::task]
async fn net_task(mut runner: embassy_net::Runner<'static, cyw43::NetDriver<'static>>) -> ! {
    runner.run().await
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    info!("WiFi Blinky with Telemetry Starting!");

    let p = embassy_rp::init(Default::default());
    let mut led = Output::new(p.PIN_16, Level::Low);
    let mut rng = RoscRng;

    let fw = include_bytes!("../cyw43-firmware/43439A0.bin");
    let clm = include_bytes!("../cyw43-firmware/43439A0_clm.bin");

    let pwr = Output::new(p.PIN_23, Level::Low);
    let cs = Output::new(p.PIN_25, Level::High);
    let mut pio = Pio::new(p.PIO0, drivers::wifi::Irqs);
    let spi = PioSpi::new(
        &mut pio.common,
        pio.sm0,
        DEFAULT_CLOCK_DIVIDER,
        pio.irq0,
        cs,
        p.PIN_24,
        p.PIN_29,
        p.DMA_CH0,
    );

    static STATE: StaticCell<cyw43::State> = StaticCell::new();
    let state = STATE.init(cyw43::State::new());

    let (net_device, mut control, runner) = cyw43::new(state, pwr, spi, fw).await;
    spawner.spawn(cyw43_task(runner)).unwrap();

    control.init(clm).await;
    control
        .set_power_management(cyw43::PowerManagementMode::PowerSave)
        .await;

    let config = Config::dhcpv4(Default::default());
    let seed = rng.next_u64();

    static RESOURCES: StaticCell<StackResources<5>> = StaticCell::new();
    let (stack, runner) = embassy_net::new(
        net_device,
        config,
        RESOURCES.init(StackResources::new()),
        seed,
    );

    spawner.spawn(net_task(runner)).unwrap();

    // Try to connect to WiFi with retries
    let mut wifi_retry_count = 0;
    const MAX_WIFI_RETRIES: u8 = 10;

    loop {
        info!(
            "Attempting to connect to WiFi (attempt {})",
            wifi_retry_count + 1
        );
        match control
            .join(WIFI_NETWORK, JoinOptions::new(WIFI_PASSWORD.as_bytes()))
            .await
        {
            Ok(_) => {
                info!("WiFi connected successfully!");
                break;
            }
            Err(err) => {
                warn!("WiFi join failed with status={}", err.status);
                wifi_retry_count += 1;
                if wifi_retry_count >= MAX_WIFI_RETRIES {
                    defmt::panic!(
                        "Failed to connect to WiFi after {} attempts",
                        MAX_WIFI_RETRIES
                    );
                }
                Timer::after(Duration::from_secs(5)).await;
            }
        }
    }

    // Wait for DHCP with timeout
    info!("Waiting for DHCP...");
    let mut dhcp_timeout = 30; // 30 seconds timeout
    while !stack.is_config_up() && dhcp_timeout > 0 {
        Timer::after(Duration::from_secs(1)).await;
        dhcp_timeout -= 1;
    }

    if !stack.is_config_up() {
        defmt::panic!("DHCP failed - no IP address assigned");
    }

    info!("DHCP is now up!");

    // Wait for link up
    info!("Waiting for link up...");
    while !stack.is_link_up() {
        Timer::after(Duration::from_millis(500)).await;
    }
    info!("Link is up!");

    // Wait for stack to be up
    info!("Waiting for stack to be up...");
    stack.wait_config_up().await;
    info!("Stack is up!");

    // Initialize the telemetry task
    let telemetry_task_config = TelemetryTaskConfig {
        interval_seconds: 60,
        retry_delay_seconds: 5,
        max_retry_attempts: 3,
        enable_backoff: true,
        max_backoff_seconds: 300,
    };

    // Spawn the telemetry task with the stack
    spawner
        .spawn(telemetry_with_retry_task(stack, telemetry_task_config))
        .unwrap();

    // Main loop - blink LED and keep main task alive
    let mut telemetry_interval = 0;
    const TELEMETRY_SEND_EVERY: u32 = 30; // Send telemetry every 30 blinks (30 seconds)

    loop {
        led.set_high();
        Timer::after(Duration::from_millis(500)).await;

        // Send telemetry less frequently to avoid overwhelming the server
        if telemetry_interval % TELEMETRY_SEND_EVERY == 0 {
            info!("Sending telemetry...");
        }

        telemetry_interval += 1;

        led.set_low();
        Timer::after(Duration::from_millis(500)).await;
    }
}
