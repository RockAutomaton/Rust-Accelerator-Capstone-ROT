use embassy_rp::gpio::{AnyPin, Level, Output};
use embassy_time::Duration;

pub struct Led<'a> {
    pin: Output<'a>,
}

impl<'a> Led<'a> {
    pub fn new(pin: AnyPin) -> Self {
        Self {
            pin: Output::new(pin, Level::Low),
        }
    }

    pub fn set_high(&mut self) {
        self.pin.set_high();
    }

    pub fn set_low(&mut self) {
        self.pin.set_low();
    }
}
