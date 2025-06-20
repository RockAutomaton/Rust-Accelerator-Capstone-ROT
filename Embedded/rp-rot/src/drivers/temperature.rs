/// # Temperature Sensor Driver
///
/// This module provides a driver for the RP2040's internal temperature sensor.
/// It allows reading temperature values in Celsius, raw ADC values, and voltage.

use defmt::*;
use embassy_rp::adc::{Adc, Channel, Config};
use embassy_rp::peripherals::{ADC, ADC_TEMP_SENSOR};
use {defmt_rtt as _, panic_probe as _};

/// Driver for the RP2040's internal temperature sensor.
/// 
/// This struct encapsulates the ADC peripheral and temperature sensor channel
/// to provide easy access to temperature readings.
pub struct TemperatureSensor {
    /// The Analog-to-Digital Converter peripheral
    adc: Adc<'static, embassy_rp::adc::Async>,
    
    /// The temperature sensor ADC channel
    channel: Channel<'static>,
}

impl TemperatureSensor {
    /// Creates a new temperature sensor driver instance.
    /// 
    /// # Parameters
    /// * `adc` - The ADC peripheral
    /// * `temp_sensor` - The temperature sensor peripheral
    /// 
    /// # Returns
    /// A new `TemperatureSensor` instance
    pub fn new(adc: ADC, temp_sensor: ADC_TEMP_SENSOR) -> Self {
        info!("Creating new temperature sensor driver");
        // Initialize the ADC with default configuration
        let adc = Adc::new(adc, crate::Irqs, Config::default());
        // Create a channel for the temperature sensor
        let channel = Channel::new_temp_sensor(temp_sensor);

        Self { adc, channel }
    }

    /// Reads the current temperature from the sensor.
    /// 
    /// # Returns
    /// * `Ok(f32)` - The temperature in degrees Celsius
    /// * `Err` - ADC error if reading fails
    pub async fn read_temperature(&mut self) -> Result<f32, embassy_rp::adc::Error> {
        // Read raw ADC value
        let raw = self.adc.read(&mut self.channel).await?;

        // Convert raw ADC value to temperature in Celsius
        // Formula from RP2040 datasheet: T = 27 - (ADC_voltage - 0.706) / 0.001721
        // 1. Convert raw ADC value to voltage (ADC is 12-bit, so max value is 4095)
        let voltage = raw as f32 * 3.3 / 4096.0;
        // 2. Apply the calibration formula
        let temp_celsius = 27.0 - (voltage - 0.706) / 0.001721;

        info!("Temperature reading: {}°C", temp_celsius);
        Ok(temp_celsius)
    }

    /// Reads the raw ADC value from the temperature sensor.
    /// 
    /// # Returns
    /// * `Ok(u16)` - The raw ADC value (0-4095)
    /// * `Err` - ADC error if reading fails
    pub async fn read_raw(&mut self) -> Result<u16, embassy_rp::adc::Error> {
        // Read and return the raw ADC value
        let raw = self.adc.read(&mut self.channel).await?;
        info!("Raw ADC reading: {}", raw);
        Ok(raw)
    }

    /// Reads the voltage from the temperature sensor.
    /// 
    /// # Returns
    /// * `Ok(f32)` - The voltage in volts
    /// * `Err` - ADC error if reading fails
    pub async fn read_voltage(&mut self) -> Result<f32, embassy_rp::adc::Error> {
        // Read raw ADC value
        let raw = self.adc.read(&mut self.channel).await?;
        // Convert to voltage (ADC is 12-bit, reference voltage is 3.3V)
        let voltage = raw as f32 * 3.3 / 4096.0;
        info!("Voltage reading: {}V", voltage);
        Ok(voltage)
    }
}
