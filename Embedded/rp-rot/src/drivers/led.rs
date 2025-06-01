use defmt::*;
use embassy_rp::gpio::{AnyPin, Level, Output};
use embassy_time::{Duration, Timer};

pub struct Led {
    pin: Output<'static>,
}

impl Led {
    pub fn new(pin: AnyPin) -> Self {
        info!("Creating new LED driver");
        Self {
            pin: Output::new(pin, Level::Low),
        }
    }

    pub async fn blink(&mut self) {
        info!("LED on");
        self.pin.set_high();
        Timer::after(Duration::from_millis(500)).await;
        info!("LED off");
        self.pin.set_low();
        Timer::after(Duration::from_millis(500)).await;
    }

    pub async fn error_blink(&mut self) {
        info!("Starting error blink pattern");
        for _ in 0..3 {
            self.pin.set_high();
            Timer::after(Duration::from_millis(100)).await;
            self.pin.set_low();
            Timer::after(Duration::from_millis(100)).await;
        }
    }

    pub async fn success_blink(&mut self) {
        info!("Starting success blink pattern");
        for _ in 0..5 {
            self.pin.set_high();
            Timer::after(Duration::from_millis(100)).await;
            self.pin.set_low();
            Timer::after(Duration::from_millis(100)).await;
        }
    }

    pub fn set_high(&mut self) {
        self.pin.set_high();
    }

    pub fn set_low(&mut self) {
        self.pin.set_low();
    }
}
