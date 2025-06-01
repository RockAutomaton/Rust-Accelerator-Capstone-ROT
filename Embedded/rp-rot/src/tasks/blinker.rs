use crate::drivers::Led;
use defmt::*;
use embassy_time::{Duration, Timer};

#[embassy_executor::task]
pub async fn blinker_task(mut led: Led<'static>) -> ! {
    info!("Starting LED blinker task");
    loop {
        led.set_high();
        Timer::after(Duration::from_millis(1000)).await;
        led.set_low();
        Timer::after(Duration::from_millis(1000)).await;
    }
}
