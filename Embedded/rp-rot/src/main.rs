//! Embassy-based blinky with WiFi capability
#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_rp::gpio::{Level, Output};
use embassy_time::{Duration, Timer};
use {defmt_rtt as _, panic_probe as _};

// WiFi-related imports (you'll use these when adding WiFi)
// use cyw43_pio::PioSpi;
// use embassy_rp::bind_interrupts;
// use embassy_rp::peripherals::{DMA_CH0, PIN_23, PIN_25, PIO0};
// use embassy_rp::pio::{InterruptHandler, Pio};

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_rp::init(Default::default());
    info!("Embassy initialized!");

    // Spawn the blinky task
    spawner.spawn(blink_task(p.PIN_16)).unwrap();
    
    // Spawn WiFi task when ready
    // spawner.spawn(wifi_task()).unwrap();

    // Keep the main task alive
    loop {
        Timer::after(Duration::from_secs(1)).await;
    }
}

#[embassy_executor::task]
async fn blink_task(pin: embassy_rp::peripherals::PIN_16) {
    let mut led = Output::new(pin, Level::Low);
    
    loop {
        info!("LED on!");
        led.set_high();
        Timer::after(Duration::from_millis(500)).await;
        
        info!("LED off!");
        led.set_low();
        Timer::after(Duration::from_millis(500)).await;
    }
}

// WiFi task template - uncomment and modify when ready
/*
#[embassy_executor::task]
async fn wifi_task() {
    // WiFi initialization code will go here
    info!("WiFi task started");
    loop {
        Timer::after(Duration::from_secs(10)).await;
        info!("WiFi task tick");
    }
}
*/