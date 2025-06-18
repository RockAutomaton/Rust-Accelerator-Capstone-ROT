//! WiFi Telemetry System - Modular Version
#![no_std]
#![no_main]

use cyw43::JoinOptions;
use cyw43_pio::{PioSpi, DEFAULT_CLOCK_DIVIDER};
use defmt::*;
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_net::{Config, StackResources};
use embassy_rp::adc::InterruptHandler as AdcInterruptHandler;
use embassy_rp::bind_interrupts;
use embassy_rp::clocks::RoscRng;
use embassy_rp::gpio::{Level, Output};
use embassy_rp::peripherals::*;
use embassy_rp::pio::{InterruptHandler as PioInterruptHandler, Pio};
use embassy_time::{Duration, Timer};
use panic_probe as _;
use rand_core::RngCore;
use static_cell::StaticCell;

// Import our modules
mod config;
mod drivers;
mod error;
mod network;
mod tasks;
mod utils;

use drivers::{Led, TemperatureSensor};
use tasks::config_fetch_task;
use tasks::{cyw43_task, network_task, telemetry_task, TelemetryTaskConfig};
use utils::config_store::get_device_config;
use utils::config_store::init_config_store;
use utils::debug_server::post_to_debug_server;

use embassy_rp::gpio::AnyPin;

// WiFi credentials from build-time environment variables
const WIFI_NETWORK: &str = env!("WIFI_NETWORK");
const WIFI_PASSWORD: &str = env!("WIFI_PASSWORD");

bind_interrupts!(struct Irqs {
    PIO0_IRQ_0 => PioInterruptHandler<PIO0>;
    ADC_IRQ_FIFO => AdcInterruptHandler;
});

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    info!("WiFi Telemetry System - Starting!");

    let p = embassy_rp::init(Default::default());
    let mut rng = RoscRng;

    // Initialize LED
    info!("Initializing LED...");
    let mut led = Led::new(AnyPin::from(p.PIN_16));
    led.error_blink().await; // Startup pattern

    // Initialize temperature sensor
    info!("Initializing temperature sensor...");
    let temp_sensor = TemperatureSensor::new(p.ADC, p.ADC_TEMP_SENSOR);

    // Initialize WiFi
    info!("Initializing WiFi...");
    let fw = include_bytes!("../cyw43-firmware/43439A0.bin");
    let clm = include_bytes!("../cyw43-firmware/43439A0_clm.bin");

    let pwr = Output::new(p.PIN_23, Level::Low);
    let cs = Output::new(p.PIN_25, Level::High);
    let mut pio = Pio::new(p.PIO0, Irqs);
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

    // Initialize network stack
    let config = Config::dhcpv4(Default::default());
    let seed = rng.next_u64();

    static RESOURCES: StaticCell<StackResources<5>> = StaticCell::new();
    let (stack, runner) = embassy_net::new(
        net_device,
        config,
        RESOURCES.init(StackResources::new()),
        seed,
    );

    spawner.spawn(network_task(runner)).unwrap();

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
                led.success_blink().await;
                break;
            }
            Err(err) => {
                warn!("WiFi join failed with status={}", err.status);
                wifi_retry_count += 1;
                if wifi_retry_count >= MAX_WIFI_RETRIES {
                    error!(
                        "Failed to connect to WiFi after {} attempts",
                        MAX_WIFI_RETRIES
                    );
                    loop {
                        led.error_blink().await;
                    }
                }
                Timer::after(Duration::from_secs(5)).await;
            }
        }
    }

    // Wait for DHCP with timeout
    info!("Waiting for DHCP...");
    let _ = post_to_debug_server(&stack, "Waiting for DHCP...").await;
    let mut dhcp_timeout = 30; // 30 seconds timeout
    while !stack.is_config_up() && dhcp_timeout > 0 {
        Timer::after(Duration::from_secs(1)).await;
        dhcp_timeout -= 1;
    }

    if !stack.is_config_up() {
        error!("DHCP failed - no IP address assigned");
        loop {
            led.error_blink().await;
        }
    }
    info!("DHCP is now up!");
    let _ = post_to_debug_server(&stack, "DHCP is now up!").await;

    info!("Waiting for link up...");
    let _ = post_to_debug_server(&stack, "Waiting for link up...").await;
    while !stack.is_link_up() {
        Timer::after(Duration::from_millis(500)).await;
    }
    info!("Link is up!");
    let _ = post_to_debug_server(&stack, "Link is up!").await;

    info!("Waiting for stack to be up...");
    let _ = post_to_debug_server(&stack, "Waiting for stack to be up...").await;
    stack.wait_config_up().await;
    info!("Stack is up!");
    let _ = post_to_debug_server(&stack, "Stack is up!").await;

    // Initialize config store
    init_config_store();

    // Spawn config fetch task
    spawner.spawn(config_fetch_task(stack)).unwrap();

    // Initialize telemetry task
    let telemetry_task_config = TelemetryTaskConfig {
        interval_seconds: 30,
    };

    spawner
        .spawn(telemetry_task(stack, telemetry_task_config, temp_sensor))
        .unwrap();

    // Main loop - poll config and set LED accordingly
    loop {
        if let Some(config) = get_device_config().await {
            if let Some(led_state) = config.config.LED.as_deref() {
                match led_state {
                    "on" => led.set_high(),
                    "off" => led.set_low(),
                    _ => { /* Optionally handle unknown state */ }
                }
            }
        }
        // Poll every second
        Timer::after(Duration::from_secs(1)).await;
    }
}
