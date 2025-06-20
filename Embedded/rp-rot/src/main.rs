//! # WiFi Telemetry System for Raspberry Pi Pico
//! 
//! This firmware implements a telemetry system for the Raspberry Pi Pico (RP2040)
//! that collects sensor data and transmits it to a cloud backend via WiFi.
//! It also receives and applies configuration updates from the cloud.
//!
//! ## Features
//! - Temperature sensing using the RP2040's internal temperature sensor
//! - Voltage monitoring
//! - WiFi connectivity through the CYW43 chipset
//! - LED status indicators
//! - Configuration management with remote updates
//! - Async operation using Embassy framework

#![no_std]  // No standard library (embedded environment)
#![no_main]  // No standard main function entry point

// External crate imports
use cyw43::JoinOptions;
use cyw43_pio::{PioSpi, DEFAULT_CLOCK_DIVIDER};
use defmt::*;  // Logging macros
use defmt_rtt as _;  // RTT (Real-Time Transfer) for logging output
use embassy_executor::Spawner;
use embassy_net::{Config, StackResources};
use embassy_rp::adc::InterruptHandler as AdcInterruptHandler;
use embassy_rp::bind_interrupts;
use embassy_rp::clocks::RoscRng;  // Ring oscillator-based random number generator
use embassy_rp::gpio::{Level, Output};
use embassy_rp::peripherals::*;
use embassy_rp::pio::{InterruptHandler as PioInterruptHandler, Pio};
use embassy_time::{Duration, Timer};
use panic_probe as _;  // Panic handler that outputs to debug probe
use rand_core::RngCore;
use static_cell::StaticCell;  // For static allocation of memory

// Import our local modules
mod config;    // Device and system configuration
mod drivers;   // Hardware abstraction layer
mod error;     // Error types and handling
mod network;   // Network communication
mod tasks;     // Async tasks for different device functions
mod utils;     // Utility functions and helpers

// Import specific components from our modules
use drivers::{Led, TemperatureSensor};
use tasks::config_fetch_task;
use tasks::{cyw43_task, network_task, telemetry_task, TelemetryTaskConfig};
use utils::config_store::get_device_config;
use utils::config_store::init_config_store;
use utils::debug_server::post_to_debug_server;

// Import additional required types
use embassy_rp::gpio::AnyPin;

// WiFi credentials are stored as environment variables and included at build time
// This avoids hardcoding sensitive information in the source code
const WIFI_NETWORK: &str = env!("WIFI_NETWORK");
const WIFI_PASSWORD: &str = env!("WIFI_PASSWORD");

// Bind hardware interrupts to our interrupt handlers
// This is required for the PIO (used by WiFi) and ADC (used by temperature sensor)
bind_interrupts!(struct Irqs {
    PIO0_IRQ_0 => PioInterruptHandler<PIO0>;
    ADC_IRQ_FIFO => AdcInterruptHandler;
});

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    // Log startup message
    info!("WiFi Telemetry System - Starting!");

    // Initialize the RP2040 peripherals with default settings
    let p = embassy_rp::init(Default::default());
    
    // Create a random number generator based on the ring oscillator
    // This is used for network stack initialization
    let mut rng = RoscRng;

    // ======== Initialize LED ========
    info!("Initializing LED...");
    // Create LED driver connected to GPIO pin 16
    let mut led = Led::new(AnyPin::from(p.PIN_16));
    // Display startup pattern to indicate we're booting
    led.error_blink().await;

    // ======== Initialize Temperature Sensor ========
    info!("Initializing temperature sensor...");
    // Create temperature sensor driver using the internal RP2040 temperature sensor
    let temp_sensor = TemperatureSensor::new(p.ADC, p.ADC_TEMP_SENSOR);

    // ======== Initialize WiFi ========
    info!("Initializing WiFi...");
    // Include WiFi firmware and CLM (Country Locale Matrix) data
    // These binary files are required to operate the CYW43 WiFi chip
    let fw = include_bytes!("../cyw43-firmware/43439A0.bin");
    let clm = include_bytes!("../cyw43-firmware/43439A0_clm.bin");

    // Configure WiFi pins and interfaces
    // PWR is the power pin for the WiFi chip (active low)
    let pwr = Output::new(p.PIN_23, Level::Low);
    // CS is the chip select pin for SPI communication (active low)
    let cs = Output::new(p.PIN_25, Level::High);
    
    // Initialize the PIO (Programmable I/O) block for SPI communication with WiFi chip
    let mut pio = Pio::new(p.PIO0, Irqs);
    
    // Configure PIO-based SPI for WiFi communication
    let spi = PioSpi::new(
        &mut pio.common,
        pio.sm0,                // Use state machine 0
        DEFAULT_CLOCK_DIVIDER,  // Default SPI clock speed
        pio.irq0,               // Interrupt for SPI completion
        cs,                     // Chip select pin
        p.PIN_24,               // SPI clock pin
        p.PIN_29,               // SPI data pin
        p.DMA_CH0,              // DMA channel for data transfer
    );

    // Create static storage for WiFi state
    // This must be static as it needs to outlive the main function
    static STATE: StaticCell<cyw43::State> = StaticCell::new();
    let state = STATE.init(cyw43::State::new());

    // Initialize the CYW43 WiFi driver
    let (net_device, mut control, runner) = cyw43::new(state, pwr, spi, fw).await;
    
    // Spawn a task to run the WiFi driver
    // This task handles WiFi communication in the background
    spawner.spawn(cyw43_task(runner)).unwrap();

    // Initialize the WiFi chip with the CLM data
    control.init(clm).await;
    
    // Set power management mode to reduce power consumption
    control
        .set_power_management(cyw43::PowerManagementMode::PowerSave)
        .await;

    // ======== Initialize Network Stack ========
    // Configure network stack to use DHCP for IP address assignment
    let config = Config::dhcpv4(Default::default());
    
    // Generate a random seed for the network stack
    // This is used for things like TCP sequence numbers
    let seed = rng.next_u64();

    // Create static storage for network stack resources
    // The '5' here defines the maximum number of sockets that can be open simultaneously
    static RESOURCES: StaticCell<StackResources<5>> = StaticCell::new();
    
    // Initialize the network stack with our device, config, resources, and seed
    let (stack, runner) = embassy_net::new(
        net_device,
        config,
        RESOURCES.init(StackResources::new()),
        seed,
    );

    // Spawn a task to run the network stack
    // This task handles TCP/IP communication in the background
    spawner.spawn(network_task(runner)).unwrap();

    // ======== Connect to WiFi with Retries ========
    let mut wifi_retry_count = 0;
    const MAX_WIFI_RETRIES: u8 = 10;  // Maximum number of connection attempts

    // Loop until we connect or exhaust all retries
    loop {
        info!(
            "Attempting to connect to WiFi (attempt {})",
            wifi_retry_count + 1
        );
        
        // Attempt to join the WiFi network
        match control
            .join(WIFI_NETWORK, JoinOptions::new(WIFI_PASSWORD.as_bytes()))
            .await
        {
            Ok(_) => {
                // Connection successful
                info!("WiFi connected successfully!");
                led.success_blink().await;  // Visual indicator of successful connection
                break;  // Exit the retry loop
            }
            Err(err) => {
                // Connection failed
                warn!("WiFi join failed with status={}", err.status);
                wifi_retry_count += 1;
                
                // If we've exhausted all retries, enter error state
                if wifi_retry_count >= MAX_WIFI_RETRIES {
                    error!(
                        "Failed to connect to WiFi after {} attempts",
                        MAX_WIFI_RETRIES
                    );
                    // Infinite error blink loop - device needs reset at this point
                    loop {
                        led.error_blink().await;
                    }
                }
                
                // Wait before retrying
                Timer::after(Duration::from_secs(5)).await;
            }
        }
    }

    // ======== Wait for DHCP with Timeout ========
    info!("Waiting for DHCP...");
    // Send a debug message to our debug server if available
    let _ = post_to_debug_server(&stack, "Waiting for DHCP...").await;
    
    // Set a timeout for DHCP to complete (30 seconds)
    let mut dhcp_timeout = 30; 
    
    // Wait for DHCP to assign an IP address
    while !stack.is_config_up() && dhcp_timeout > 0 {
        Timer::after(Duration::from_secs(1)).await;
        dhcp_timeout -= 1;
    }

    // If DHCP failed after timeout, enter error state
    if !stack.is_config_up() {
        error!("DHCP failed - no IP address assigned");
        // Infinite error blink loop - device needs reset at this point
        loop {
            led.error_blink().await;
        }
    }
    
    // DHCP successful - we have an IP address
    info!("DHCP is now up!");
    let _ = post_to_debug_server(&stack, "DHCP is now up!").await;

    // ======== Wait for Network Link ========
    info!("Waiting for link up...");
    let _ = post_to_debug_server(&stack, "Waiting for link up...").await;
    
    // Poll link status until it's up
    while !stack.is_link_up() {
        Timer::after(Duration::from_millis(500)).await;
    }
    
    info!("Link is up!");
    let _ = post_to_debug_server(&stack, "Link is up!").await;

    // ======== Wait for Network Stack ========
    info!("Waiting for stack to be up...");
    let _ = post_to_debug_server(&stack, "Waiting for stack to be up...").await;
    
    // This will wait until the network stack is fully configured and ready
    stack.wait_config_up().await;
    
    info!("Stack is up!");
    let _ = post_to_debug_server(&stack, "Stack is up!").await;

    // ======== Initialize Configuration Store ========
    // This initializes the persistent storage for device configuration
    init_config_store();

    // ======== Spawn Configuration Fetch Task ========
    // This task periodically fetches configuration updates from the cloud
    spawner.spawn(config_fetch_task(stack)).unwrap();

    // ======== Initialize and Spawn Telemetry Task ========
    // Configure the telemetry task to send data every 30 seconds
    let telemetry_task_config = TelemetryTaskConfig {
        interval_seconds: 30,
    };

    // Spawn the telemetry task that will collect and send sensor data
    spawner
        .spawn(telemetry_task(stack, telemetry_task_config, temp_sensor))
        .unwrap();

    // ======== Main Loop - Apply Configuration ========
    // This is the main application loop that runs continuously
    loop {
        // Check if we have a valid device configuration
        if let Some(config) = get_device_config().await {
            // Look for LED configuration
            if let Some(led_state) = config.config.LED.as_deref() {
                // Apply LED state based on configuration
                match led_state {
                    "on" => led.set_high(),  // Turn LED on
                    "off" => led.set_low(),  // Turn LED off
                    _ => { /* Ignore unknown states */ }
                }
            }
        }
        // Poll configuration every second
        Timer::after(Duration::from_secs(1)).await;
    }
}
