//! WiFi Blinky - LED blinks when WiFi is connected
#![no_std]
#![no_main]

use cyw43::JoinOptions;
use cyw43_pio::{PioSpi, DEFAULT_CLOCK_DIVIDER};
use defmt::*;
use embassy_executor::Spawner;
use embassy_net::{Config, StackResources, Ipv4Address};
use embassy_net::tcp::TcpSocket;
use embassy_rp::bind_interrupts;
use embassy_rp::clocks::RoscRng;
use embassy_rp::gpio::{Level, Output};
use embassy_rp::peripherals::{DMA_CH0, PIO0};
use embassy_rp::pio::{InterruptHandler, Pio};
use embassy_time::{Duration, Timer};
use rand_core::RngCore;
use static_cell::StaticCell;
use {defmt_rtt as _, panic_probe as _};

// Import WiFi credentials from separate file
mod wifi_credentials;
use wifi_credentials::*;

// Convert IP array to Ipv4Address
const TEST_IP_ADDR: Ipv4Address = Ipv4Address::new(TEST_IP[0], TEST_IP[1], TEST_IP[2], TEST_IP[3]);

bind_interrupts!(struct Irqs {
    PIO0_IRQ_0 => InterruptHandler<PIO0>;
});

#[embassy_executor::task]
async fn cyw43_task(runner: cyw43::Runner<'static, Output<'static>, PioSpi<'static, PIO0, 0, DMA_CH0>>) -> ! {
    runner.run().await
}

#[embassy_executor::task]
async fn net_task(mut runner: embassy_net::Runner<'static, cyw43::NetDriver<'static>>) -> ! {
    runner.run().await
}

async fn show_status(led: &mut Output<'_>, flashes: u8, delay_ms: u64) {
    for _ in 0..flashes {
        led.set_high();
        Timer::after(Duration::from_millis(delay_ms)).await;
        led.set_low();
        Timer::after(Duration::from_millis(delay_ms)).await;
    }
    Timer::after(Duration::from_millis(500)).await;
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    info!("WiFi Blinky Starting!");

    let p = embassy_rp::init(Default::default());
    let mut led = Output::new(p.PIN_16, Level::Low);
    let mut rng = RoscRng;

    // 1 flash: Starting
    show_status(&mut led, 1, 200).await;

    let fw = include_bytes!("../cyw43-firmware/43439A0.bin");
    let clm = include_bytes!("../cyw43-firmware/43439A0_clm.bin");

    // 2 flashes: Got firmware files
    show_status(&mut led, 2, 200).await;

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

    // 3 flashes: Hardware setup done
    show_status(&mut led, 3, 200).await;

    static STATE: StaticCell<cyw43::State> = StaticCell::new();
    let state = STATE.init(cyw43::State::new());
    
    let (net_device, mut control, runner) = cyw43::new(state, pwr, spi, fw).await;
    spawner.spawn(cyw43_task(runner)).unwrap();

    // 4 flashes: WiFi chip initialized!
    show_status(&mut led, 4, 200).await;

    control.init(clm).await;
    control
        .set_power_management(cyw43::PowerManagementMode::PowerSave)
        .await;

    // 5 flashes: WiFi control initialized
    show_status(&mut led, 5, 200).await;

    let config = Config::dhcpv4(Default::default());
    let seed = rng.next_u64();

    static RESOURCES: StaticCell<StackResources<5>> = StaticCell::new();
    let (stack, runner) = embassy_net::new(net_device, config, RESOURCES.init(StackResources::new()), seed);

    spawner.spawn(net_task(runner)).unwrap();

    // 6 flashes: Network stack ready
    show_status(&mut led, 6, 200).await;

    // Try to connect to WiFi
    loop {
        match control
            .join(WIFI_NETWORK, JoinOptions::new(WIFI_PASSWORD.as_bytes()))
            .await
        {
            Ok(_) => break,
            Err(err) => {
                info!("join failed with status={}", err.status);
                show_status(&mut led, 1, 100).await;
                Timer::after(Duration::from_secs(2)).await;
            }
        }
    }

    // 7 flashes: WiFi connected!
    show_status(&mut led, 7, 200).await;

    // Wait for DHCP
    info!("waiting for DHCP...");
    while !stack.is_config_up() {
        Timer::after(Duration::from_millis(100)).await;
    }
    info!("DHCP is now up!");

    // 8 flashes: DHCP configured
    show_status(&mut led, 8, 200).await;

    info!("waiting for link up...");
    while !stack.is_link_up() {
        Timer::after(Duration::from_millis(500)).await;
    }
    info!("Link is up!");

    // 9 flashes: Link is up
    show_status(&mut led, 9, 200).await;

    info!("waiting for stack to be up...");
    stack.wait_config_up().await;
    info!("Stack is up!");

    // 10 flashes: Everything ready - now just blink to show WiFi is working!
    show_status(&mut led, 10, 200).await;

    // SUCCESS! WiFi is connected - steady heartbeat blink
    loop {
        led.set_high();
        Timer::after(Duration::from_millis(500)).await;
        led.set_low();
        Timer::after(Duration::from_millis(500)).await;
    }
}