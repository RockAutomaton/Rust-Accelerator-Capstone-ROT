/// # LED Blinker Task
///
/// This module defines a task that blinks an LED in a continuous pattern.
/// It provides a visual indication that the device is running and can be
/// used for status indication.

use crate::drivers::Led;
use defmt::*;
use embassy_time::{Duration, Timer};

/// Embassy task for controlling the LED blinking pattern.
///
/// This task runs in a continuous loop, blinking the LED at regular intervals.
/// It provides a visual heartbeat to show that the device is running.
///
/// # Parameters
/// * `led` - LED driver instance to control
///
/// # Note
/// This function never returns as it's designed to run for the entire
/// device lifecycle.
#[embassy_executor::task]
pub async fn blinker_task(mut led: Led) -> ! {
    info!("Starting LED blinker task");
    
    // Infinite loop to continuously blink the LED
    loop {
        // Execute one blink cycle (LED on, then off)
        led.blink().await;
        
        // The timing is handled inside the blink method
    }
}
