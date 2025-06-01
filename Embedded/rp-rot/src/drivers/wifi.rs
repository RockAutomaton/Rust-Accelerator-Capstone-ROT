use crate::config::WiFiConfig;
use crate::error::WiFiError;
use cyw43::{JoinOptions, PowerManagementMode};
use cyw43_pio::{PioSpi, DEFAULT_CLOCK_DIVIDER};
use defmt::*;
use embassy_rp::gpio::{Level, Output};
use embassy_rp::peripherals::{DMA_CH0, PIO0};
use embassy_rp::pio::{InterruptHandler, Pio};
use embassy_rp::{bind_interrupts, peripherals::*};
use embassy_time::{Duration, Timer};
use static_cell::StaticCell;

bind_interrupts!(pub struct Irqs {
    PIO0_IRQ_0 => InterruptHandler<PIO0>;
});

pub struct WiFiDriver;

impl WiFiDriver {
    pub async fn init(
        pio0: PIO0,
        pin_pwr: PIN_23,
        pin_cs: PIN_25,
        pin_dio: PIN_24,
        pin_clk: PIN_29,
        dma_ch0: DMA_CH0,
    ) -> (
        cyw43::NetDriver<'static>,
        cyw43::Control<'static>,
        cyw43::Runner<'static, Output<'static>, PioSpi<'static, PIO0, 0, DMA_CH0>>,
    ) {
        info!("Initializing CYW43 WiFi hardware...");

        let fw = include_bytes!("../../cyw43-firmware/43439A0.bin");
        let clm = include_bytes!("../../cyw43-firmware/43439A0_clm.bin");

        let pwr = Output::new(pin_pwr, Level::Low);
        let cs = Output::new(pin_cs, Level::High);
        let mut pio = Pio::new(pio0, Irqs);
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

        static STATE: StaticCell<cyw43::State> = StaticCell::new();
        let state = STATE.init(cyw43::State::new());

        let (net_device, mut control, runner) = cyw43::new(state, pwr, spi, fw).await;
        control.init(clm).await;
        control
            .set_power_management(PowerManagementMode::PowerSave)
            .await;

        // Give the CYW43 some time to stabilize
        Timer::after(Duration::from_secs(2)).await;

        (net_device, control, runner)
    }

    pub async fn connect_with_retry(
        control: &mut cyw43::Control<'_>,
        config: &WiFiConfig,
    ) -> Result<(), WiFiError> {
        if config.network.is_empty() || config.password.is_empty() {
            return Err(WiFiError::InvalidCredentials);
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
                        return Err(WiFiError::MaxRetriesExceeded);
                    }

                    Timer::after(Duration::from_secs(config.retry_delay_secs)).await;
                }
            }
        }
    }
}
