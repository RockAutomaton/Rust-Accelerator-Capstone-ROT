/// # WiFi Driver
///
/// This module provides a driver for the CYW43 WiFi chip on the Raspberry Pi Pico W.
/// It handles initialization, configuration, and connection to WiFi networks.

use cyw43::{Control, JoinOptions, PowerManagementMode};
use cyw43_pio::{PioSpi, DEFAULT_CLOCK_DIVIDER};
use defmt::*;
use embassy_rp::bind_interrupts;
use embassy_rp::gpio::{Level, Output};
use embassy_rp::peripherals::{DMA_CH0, PIN_23, PIN_24, PIN_25, PIN_29, PIO0};
use embassy_rp::pio::{InterruptHandler, Pio};
use embassy_time::{Duration, Timer};
use static_cell::StaticCell;

use crate::config::WiFiConfig;
use crate::error::WiFiError;

/// Driver for the CYW43 WiFi module.
///
/// This struct provides static methods to initialize and manage the WiFi hardware.
pub struct WiFiDriver;

impl WiFiDriver {
    /// Initializes the WiFi hardware and returns the necessary components.
    ///
    /// This function:
    /// 1. Loads the WiFi firmware
    /// 2. Initializes GPIO pins for communication
    /// 3. Sets up the PIO (Programmable I/O) for SPI communication
    /// 4. Creates and initializes the WiFi state
    /// 5. Configures power management
    ///
    /// # Parameters
    /// * `pio0` - PIO0 peripheral
    /// * `pin_pwr` - Power pin (PIN_23)
    /// * `pin_cs` - Chip select pin (PIN_25)
    /// * `pin_dio` - Data I/O pin (PIN_24)
    /// * `pin_clk` - Clock pin (PIN_29)
    /// * `dma_ch0` - DMA channel 0
    ///
    /// # Returns
    /// * `Ok(...)` - Tuple of (NetDriver, Control, Runner) on success
    /// * `Err(WiFiError)` - Error if initialization fails
    pub async fn init(
        pio0: PIO0,
        pin_pwr: PIN_23,
        pin_cs: PIN_25,
        pin_dio: PIN_24,
        pin_clk: PIN_29,
        dma_ch0: DMA_CH0,
    ) -> Result<
        (
            cyw43::NetDriver<'static>,
            Control<'static>,
            cyw43::Runner<'static, Output<'static>, PioSpi<'static, PIO0, 0, DMA_CH0>>,
        ),
        WiFiError,
    > {
        info!("Starting WiFi initialization...");

        // Load firmware and CLM (Country Locale Matrix) files
        info!("Loading WiFi firmware...");
        let fw = include_bytes!("../../cyw43-firmware/43439A0.bin");
        let clm = include_bytes!("../../cyw43-firmware/43439A0_clm.bin");

        // Verify that firmware files are not empty
        if fw.is_empty() || clm.is_empty() {
            error!("Firmware files are empty!");
            return Err(WiFiError::InitFailed);
        }
        info!(
            "Firmware loaded: fw size={}, clm size={}",
            fw.len(),
            clm.len()
        );

        // Initialize pins and hardware with delays between steps
        // These delays help ensure stable initialization

        // Initialize power pin (active low)
        info!("Initializing power pin...");
        let pwr = Output::new(pin_pwr, Level::Low);
        Timer::after(Duration::from_millis(100)).await;

        // Initialize chip select pin (active low)
        info!("Initializing CS pin...");
        let cs = Output::new(pin_cs, Level::High);
        Timer::after(Duration::from_millis(100)).await;

        // Initialize the PIO (Programmable I/O) block with interrupts
        info!("Initializing PIO...");
        let mut pio = Pio::new(pio0, crate::Irqs);
        Timer::after(Duration::from_millis(100)).await;

        // Initialize SPI communication using PIO
        info!("Initializing SPI...");
        let spi = PioSpi::new(
            &mut pio.common,       // PIO common block
            pio.sm0,               // State machine 0
            DEFAULT_CLOCK_DIVIDER, // Default SPI clock speed
            pio.irq0,              // Interrupt for SPI completion
            cs,                    // Chip select pin
            pin_dio,               // Data pin
            pin_clk,               // Clock pin
            dma_ch0,               // DMA channel for data transfer
        );
        Timer::after(Duration::from_millis(100)).await;

        // Create static storage for WiFi state
        // This must be static as it needs to outlive this function
        info!("Creating WiFi state...");
        static STATE: StaticCell<cyw43::State> = StaticCell::new();
        let state = STATE.init(cyw43::State::new());
        Timer::after(Duration::from_millis(100)).await;

        // Create the WiFi driver with the initialized components
        info!("Creating WiFi driver...");
        let (net_device, mut control, runner) = cyw43::new(state, pwr, spi, fw).await;
        Timer::after(Duration::from_millis(500)).await;

        // Initialize the WiFi control with CLM data
        info!("Initializing WiFi control...");
        control.init(clm).await;
        Timer::after(Duration::from_millis(500)).await;

        // Set power management mode to reduce power consumption
        info!("Setting power management mode...");
        control
            .set_power_management(PowerManagementMode::PowerSave)
            .await;
        Timer::after(Duration::from_millis(500)).await;

        // Return the initialized components
        info!("WiFi initialization complete");
        Ok((net_device, control, runner))
    }

    /// Connects to a WiFi network with automatic retry functionality.
    ///
    /// This function attempts to connect to the specified WiFi network, and if unsuccessful,
    /// it will retry according to the configuration parameters. This helps handle temporary
    /// connection issues that might occur during the initial connection attempt.
    ///
    /// # Parameters
    /// * `control` - WiFi control interface
    /// * `config` - WiFi configuration including network name, password, and retry settings
    ///
    /// # Returns
    /// * `Ok(())` - If connection is successful
    /// * `Err(WiFiError)` - If connection fails after all retry attempts
    pub async fn connect_with_retry(
        control: &mut Control<'_>,
        config: &WiFiConfig,
    ) -> Result<(), WiFiError> {
        // Validate that network and password are not empty
        if config.network.is_empty() || config.password.is_empty() {
            error!("WiFi network or password is empty");
            return Err(WiFiError::Join);
        }

        let mut retry_count = 0;

        // Connection attempt loop with retries
        loop {
            info!(
                "Attempting WiFi connection to '{}' (attempt {})",
                config.network,
                retry_count + 1
            );

            // Attempt to join the WiFi network
            match control
                .join(config.network, JoinOptions::new(config.password.as_bytes()))
                .await
            {
                // Connection successful
                Ok(_) => {
                    info!(
                        "Successfully connected to WiFi network '{}'",
                        config.network
                    );
                    return Ok(());
                }
                
                // Connection failed
                Err(err) => {
                    warn!(
                        "WiFi connection attempt {} failed with status={}",
                        retry_count + 1,
                        err.status
                    );

                    // Increment retry counter
                    retry_count += 1;
                    
                    // Check if we've reached the maximum number of retries
                    if retry_count >= config.max_retries {
                        error!(
                            "Failed to connect to WiFi after {} attempts",
                            config.max_retries
                        );
                        return Err(WiFiError::Timeout);
                    }

                    // Wait before the next retry attempt
                    Timer::after(Duration::from_secs(config.retry_delay_secs)).await;
                }
            }
        }
    }
}
