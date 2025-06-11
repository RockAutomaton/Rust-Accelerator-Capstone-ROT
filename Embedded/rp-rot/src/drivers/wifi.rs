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

pub struct WiFiDriver;

impl WiFiDriver {
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

        // Load firmware with verification
        info!("Loading WiFi firmware...");
        let fw = include_bytes!("../../cyw43-firmware/43439A0.bin");
        let clm = include_bytes!("../../cyw43-firmware/43439A0_clm.bin");

        if fw.is_empty() || clm.is_empty() {
            error!("Firmware files are empty!");
            return Err(WiFiError::InitFailed);
        }
        info!(
            "Firmware loaded: fw size={}, clm size={}",
            fw.len(),
            clm.len()
        );

        // Initialize pins with delays
        info!("Initializing power pin...");
        let pwr = Output::new(pin_pwr, Level::Low);
        Timer::after(Duration::from_millis(100)).await;

        info!("Initializing CS pin...");
        let cs = Output::new(pin_cs, Level::High);
        Timer::after(Duration::from_millis(100)).await;

        info!("Initializing PIO...");
        let mut pio = Pio::new(pio0, crate::Irqs);
        Timer::after(Duration::from_millis(100)).await;

        info!("Initializing SPI...");
        let spi = PioSpi::new(
            &mut pio.common,
            pio.sm0,
            DEFAULT_CLOCK_DIVIDER,
            pio.irq0,
            cs,
            pin_dio,
            pin_clk,
            dma_ch0,
        );
        Timer::after(Duration::from_millis(100)).await;

        info!("Creating WiFi state...");
        static STATE: StaticCell<cyw43::State> = StaticCell::new();
        let state = STATE.init(cyw43::State::new());
        Timer::after(Duration::from_millis(100)).await;

        info!("Creating WiFi driver...");
        let (net_device, mut control, runner) = cyw43::new(state, pwr, spi, fw).await;
        Timer::after(Duration::from_millis(500)).await;

        info!("Initializing WiFi control...");
        control.init(clm).await;
        Timer::after(Duration::from_millis(500)).await;

        info!("Setting power management mode...");
        control
            .set_power_management(PowerManagementMode::PowerSave)
            .await;
        Timer::after(Duration::from_millis(500)).await;

        info!("WiFi initialization complete");
        Ok((net_device, control, runner))
    }

    pub async fn connect_with_retry(
        control: &mut Control<'_>,
        config: &WiFiConfig,
    ) -> Result<(), WiFiError> {
        if config.network.is_empty() || config.password.is_empty() {
            return Err(WiFiError::Join);
        }

        let mut retry_count = 0;

        loop {
            info!(
                "Attempting WiFi connection to '{}' (attempt {})",
                config.network,
                retry_count + 1
            );

            match control
                .join(config.network, JoinOptions::new(config.password.as_bytes()))
                .await
            {
                Ok(_) => {
                    info!(
                        "Successfully connected to WiFi network '{}'",
                        config.network
                    );
                    return Ok(());
                }
                Err(err) => {
                    warn!(
                        "WiFi connection attempt {} failed with status={}",
                        retry_count + 1,
                        err.status
                    );

                    retry_count += 1;
                    if retry_count >= config.max_retries {
                        error!(
                            "Failed to connect to WiFi after {} attempts",
                            config.max_retries
                        );
                        return Err(WiFiError::Timeout);
                    }

                    Timer::after(Duration::from_secs(config.retry_delay_secs)).await;
                }
            }
        }
    }
}
