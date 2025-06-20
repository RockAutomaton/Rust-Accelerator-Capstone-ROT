/// # CYW43 WiFi Chip Driver Task
///
/// This module defines a task that runs the CYW43 WiFi chip driver in the background.
/// The CYW43 is the WiFi and Bluetooth chip used on the Raspberry Pi Pico W.

use cyw43_pio::PioSpi;
use defmt::*;
use embassy_rp::gpio::Output;
use embassy_rp::peripherals::{DMA_CH0, PIO0};

/// Embassy task for running the CYW43 WiFi chip driver.
///
/// This task handles low-level communication with the WiFi hardware.
/// It must run continuously in the background to maintain WiFi connectivity.
///
/// # Parameters
/// * `runner` - CYW43 runner instance that manages the WiFi chip
///
/// # Note
/// This function never returns as it's designed to run for the entire
/// device lifecycle. It should be spawned early in the initialization
/// process before any WiFi operations are attempted.
#[embassy_executor::task]
pub async fn cyw43_task(
    runner: cyw43::Runner<'static, Output<'static>, PioSpi<'static, PIO0, 0, DMA_CH0>>,
) -> ! {
    info!("Starting CYW43 WiFi chip background task");
    
    // This call blocks indefinitely, processing WiFi hardware events
    runner.run().await;
    
    // This code should never be reached under normal operation
    #[allow(unreachable_code)]
    loop {
        // Safety loop in case the runner exits unexpectedly
    }
}
