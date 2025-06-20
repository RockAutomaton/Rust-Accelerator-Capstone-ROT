/// # LED Driver
///
/// This module provides a driver for controlling an LED connected to a GPIO pin.
/// It includes functions for different blink patterns to indicate various states.

use defmt::*;
use embassy_rp::gpio::{AnyPin, Level, Output};
use embassy_time::{Duration, Timer};

/// Driver for controlling an LED.
///
/// This struct encapsulates a GPIO pin configured as an output
/// and provides methods to control an LED connected to that pin.
pub struct Led {
    /// The GPIO pin connected to the LED
    pin: Output<'static>,
}

impl Led {
    /// Creates a new LED driver instance.
    ///
    /// # Parameters
    /// * `pin` - The GPIO pin to which the LED is connected
    ///
    /// # Returns
    /// A new `Led` instance with the pin initialized as an output (initially LOW)
    pub fn new(pin: AnyPin) -> Self {
        info!("Creating new LED driver");
        Self {
            pin: Output::new(pin, Level::Low), // Initialize pin as output, initially LOW
        }
    }

    /// Performs a single blink cycle (on for 500ms, then off for 500ms).
    ///
    /// This is a basic blink pattern for general indication.
    pub async fn blink(&mut self) {
        info!("LED on");
        self.pin.set_high();                            // Turn LED on
        Timer::after(Duration::from_millis(500)).await; // Wait 500ms
        info!("LED off");
        self.pin.set_low();                             // Turn LED off
        Timer::after(Duration::from_millis(500)).await; // Wait 500ms
    }

    /// Performs an error indication blink pattern.
    ///
    /// The pattern consists of 3 quick blinks (100ms on, 100ms off)
    /// to visually indicate an error condition.
    pub async fn error_blink(&mut self) {
        info!("Starting error blink pattern");
        for _ in 0..3 {
            self.pin.set_high();                            // Turn LED on
            Timer::after(Duration::from_millis(100)).await; // Wait 100ms
            self.pin.set_low();                             // Turn LED off
            Timer::after(Duration::from_millis(100)).await; // Wait 100ms
        }
    }

    /// Performs a success indication blink pattern.
    ///
    /// The pattern consists of 5 quick blinks (100ms on, 100ms off)
    /// to visually indicate a successful operation.
    pub async fn success_blink(&mut self) {
        info!("Starting success blink pattern");
        for _ in 0..5 {
            self.pin.set_high();                            // Turn LED on
            Timer::after(Duration::from_millis(100)).await; // Wait 100ms
            self.pin.set_low();                             // Turn LED off
            Timer::after(Duration::from_millis(100)).await; // Wait 100ms
        }
    }

    /// Turns the LED on by setting the GPIO pin to HIGH.
    pub fn set_high(&mut self) {
        self.pin.set_high();
    }

    /// Turns the LED off by setting the GPIO pin to LOW.
    pub fn set_low(&mut self) {
        self.pin.set_low();
    }
}
