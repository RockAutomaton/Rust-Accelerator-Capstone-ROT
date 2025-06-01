use cyw43_pio::PioSpi;
use defmt::*;
use embassy_rp::gpio::Output;
use embassy_rp::peripherals::{DMA_CH0, PIO0};

#[embassy_executor::task]
pub async fn cyw43_task(
    runner: cyw43::Runner<'static, Output<'static>, PioSpi<'static, PIO0, 0, DMA_CH0>>,
) -> ! {
    info!("Starting CYW43 WiFi chip background task");
    runner.run().await;
}
