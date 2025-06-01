use crate::drivers::Led;
use defmt::*;
use embassy_time::{Duration, Timer};

#[embassy_executor::task]
pub async fn blinker_task(mut led: Led) -> ! {
    info!("Starting LED blinker task");
    loop {
        led.blink().await;
    }
}
