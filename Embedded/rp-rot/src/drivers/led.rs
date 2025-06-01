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
}
