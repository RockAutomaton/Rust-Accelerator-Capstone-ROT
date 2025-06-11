use defmt::*;
use embassy_rp::adc::{Adc, Channel, Config};
use embassy_rp::peripherals::{ADC, ADC_TEMP_SENSOR};
use {defmt_rtt as _, panic_probe as _};

pub struct TemperatureSensor {
    adc: Adc<'static, embassy_rp::adc::Async>,
    channel: Channel<'static>,
}

impl TemperatureSensor {
    pub fn new(adc: ADC, temp_sensor: ADC_TEMP_SENSOR) -> Self {
        info!("Creating new temperature sensor driver");
        let adc = Adc::new(adc, crate::Irqs, Config::default());
        let channel = Channel::new_temp_sensor(temp_sensor);

        Self { adc, channel }
    }

    pub async fn read_temperature(&mut self) -> Result<f32, embassy_rp::adc::Error> {
        let raw = self.adc.read(&mut self.channel).await?;

        // Convert to temperature in Celsius
        // RP2040 datasheet formula: T = 27 - (ADC_voltage - 0.706) / 0.001721
        let voltage = raw as f32 * 3.3 / 4096.0;
        let temp_celsius = 27.0 - (voltage - 0.706) / 0.001721;

        info!("Temperature reading: {}°C", temp_celsius);
        Ok(temp_celsius)
    }

    pub async fn read_raw(&mut self) -> Result<u16, embassy_rp::adc::Error> {
        let raw = self.adc.read(&mut self.channel).await?;
        info!("Raw ADC reading: {}", raw);
        Ok(raw)
    }

    pub async fn read_voltage(&mut self) -> Result<f32, embassy_rp::adc::Error> {
        let raw = self.adc.read(&mut self.channel).await?;
        let voltage = raw as f32 * 3.3 / 4096.0;
        info!("Voltage reading: {}V", voltage);
        Ok(voltage)
    }
}
