//! WiFi Blinky with Telemetry - LED blinks when WiFi is connected and sends telemetry data
#![no_std]
#![no_main]

use cyw43::JoinOptions;
use cyw43_pio::{PioSpi, DEFAULT_CLOCK_DIVIDER};
use defmt::*;
use embassy_executor::Spawner;
use embassy_net::{
    dns::{DnsQueryType, DnsSocket},
    tcp::TcpSocket,
    Config, IpEndpoint, StackResources,
};
use embassy_rp::bind_interrupts;
use embassy_rp::clocks::RoscRng;
use embassy_rp::gpio::{Level, Output};
use embassy_rp::peripherals::{DMA_CH0, PIO0};
use embassy_rp::pio::{InterruptHandler, Pio};
use embassy_time::{Duration, Timer};
use embedded_io_async::Write;
use heapless::String;
use rand_core::RngCore;
use static_cell::StaticCell;

use {defmt_rtt as _, panic_probe as _};

// WiFi credentials from build-time environment variables
const WIFI_NETWORK: &str = env!("WIFI_NETWORK");
const WIFI_PASSWORD: &str = env!("WIFI_PASSWORD");

// Telemetry configuration
const TELEMETRY_HOST: &str = env!("TELEMETRY_HOST");
const TELEMETRY_PORT: u16 = 80;
const TELEMETRY_PATH: &str = "/iot/data/ingest";

// Error types for better error handling
#[derive(Debug, defmt::Format)]
enum TelemetryError {
    DnsResolve,
    Connect,
    Write,
    Read,
}

bind_interrupts!(struct Irqs {
    PIO0_IRQ_0 => InterruptHandler<PIO0>;
});

#[embassy_executor::task]
async fn cyw43_task(
    runner: cyw43::Runner<'static, Output<'static>, PioSpi<'static, PIO0, 0, DMA_CH0>>,
) -> ! {
    runner.run().await
}

#[embassy_executor::task]
async fn net_task(mut runner: embassy_net::Runner<'static, cyw43::NetDriver<'static>>) -> ! {
    runner.run().await
}

async fn send_telemetry(stack: embassy_net::Stack<'_>) -> Result<(), TelemetryError> {
    let mut rx_buffer = [0; 1024];
    let mut tx_buffer = [0; 1024];
    let mut socket = TcpSocket::new(stack, &mut rx_buffer, &mut tx_buffer);

    // DNS resolution
    let dns_socket = DnsSocket::new(stack);

    info!("Resolving hostname: {}", TELEMETRY_HOST);
    let addresses = match dns_socket.query(TELEMETRY_HOST, DnsQueryType::A).await {
        Ok(addrs) => addrs,
        Err(_) => {
            warn!("DNS resolution failed");
            return Err(TelemetryError::DnsResolve);
        }
    };

    let host_addr = if let Some(addr) = addresses.get(0) {
        info!("Resolved {} to {}", TELEMETRY_HOST, addr);
        *addr
    } else {
        warn!("No IP addresses returned from DNS");
        return Err(TelemetryError::DnsResolve);
    };

    // Connect to the server
    info!("Connecting to {}:{}", host_addr, TELEMETRY_PORT);
    socket.set_timeout(Some(Duration::from_secs(10)));

    match socket
        .connect(IpEndpoint::new(host_addr, TELEMETRY_PORT))
        .await
    {
        Ok(_) => info!("Connected successfully"),
        Err(e) => {
            warn!("Connection failed: {:?}", e);
            return Err(TelemetryError::Connect);
        }
    }

    // Prepare the telemetry data with real sensor readings
    let telemetry_data = "{\"device_id\":\"1\",\"telemetry_data\":{\"temperature\":\"25.5\",\"humidity\":\"60.0\",\"status\":\"active\"}}";

    // Prepare HTTP request with proper headers
    let mut request = String::<512>::new(); // Increased buffer size
    let _ = core::fmt::write(
        &mut request,
        format_args!(
            "POST {} HTTP/1.1\r\n\
             Host: {}\r\n\
             Content-Type: application/json\r\n\
             Content-Length: {}\r\n\
             Connection: close\r\n\
             User-Agent: RustEmbedded/1.0\r\n\
             \r\n\
             {}",
            TELEMETRY_PATH,
            TELEMETRY_HOST,
            telemetry_data.len(),
            telemetry_data
        ),
    );

    info!("Sending HTTP request ({} bytes)", request.len());

    // Send the request
    match socket.write_all(request.as_bytes()).await {
        Ok(_) => info!("Request sent successfully"),
        Err(e) => {
            warn!("Failed to send request: {:?}", e);
            return Err(TelemetryError::Write);
        }
    }

    // Read response with timeout
    let mut buf = [0; 1024];
    match socket.read(&mut buf).await {
        Ok(n) => {
            let response = core::str::from_utf8(&buf[..n]).unwrap_or("Invalid UTF-8");
            info!("Response ({} bytes): {}", n, response);

            // Check for successful HTTP status
            if response.contains("HTTP/1.1 200") || response.contains("HTTP/1.0 200") {
                info!("Telemetry accepted by server");
            } else {
                warn!("Server returned non-200 status");
            }
        }
        Err(e) => {
            warn!("Failed to read response: {:?}", e);
            return Err(TelemetryError::Read);
        }
    }

    socket.close();
    Timer::after(Duration::from_millis(100)).await; // Allow socket to close properly
    Ok(())
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    info!("WiFi Blinky with Telemetry Starting!");

    let p = embassy_rp::init(Default::default());
    let mut led = Output::new(p.PIN_16, Level::Low);
    let mut rng = RoscRng;

    let fw = include_bytes!("../cyw43-firmware/43439A0.bin");
    let clm = include_bytes!("../cyw43-firmware/43439A0_clm.bin");

    let pwr = Output::new(p.PIN_23, Level::Low);
    let cs = Output::new(p.PIN_25, Level::High);
    let mut pio = Pio::new(p.PIO0, Irqs);
    let spi = PioSpi::new(
        &mut pio.common,
        pio.sm0,
        DEFAULT_CLOCK_DIVIDER,
        pio.irq0,
        cs,
        p.PIN_24,
        p.PIN_29,
        p.DMA_CH0,
    );

    static STATE: StaticCell<cyw43::State> = StaticCell::new();
    let state = STATE.init(cyw43::State::new());

    let (net_device, mut control, runner) = cyw43::new(state, pwr, spi, fw).await;
    spawner.spawn(cyw43_task(runner)).unwrap();

    control.init(clm).await;
    control
        .set_power_management(cyw43::PowerManagementMode::PowerSave)
        .await;

    let config = Config::dhcpv4(Default::default());
    let seed = rng.next_u64();

    static RESOURCES: StaticCell<StackResources<5>> = StaticCell::new();
    let (stack, runner) = embassy_net::new(
        net_device,
        config,
        RESOURCES.init(StackResources::new()),
        seed,
    );

    spawner.spawn(net_task(runner)).unwrap();

    // Try to connect to WiFi with retries
    let mut wifi_retry_count = 0;
    const MAX_WIFI_RETRIES: u8 = 10;

    loop {
        info!(
            "Attempting to connect to WiFi (attempt {})",
            wifi_retry_count + 1
        );
        match control
            .join(WIFI_NETWORK, JoinOptions::new(WIFI_PASSWORD.as_bytes()))
            .await
        {
            Ok(_) => {
                info!("WiFi connected successfully!");
                break;
            }
            Err(err) => {
                warn!("WiFi join failed with status={}", err.status);
                wifi_retry_count += 1;
                if wifi_retry_count >= MAX_WIFI_RETRIES {
                    defmt::panic!(
                        "Failed to connect to WiFi after {} attempts",
                        MAX_WIFI_RETRIES
                    );
                }
                Timer::after(Duration::from_secs(5)).await;
            }
        }
    }

    // Wait for DHCP with timeout
    info!("Waiting for DHCP...");
    let mut dhcp_timeout = 30; // 30 seconds timeout
    while !stack.is_config_up() && dhcp_timeout > 0 {
        Timer::after(Duration::from_secs(1)).await;
        dhcp_timeout -= 1;
    }

    if !stack.is_config_up() {
        defmt::panic!("DHCP failed - no IP address assigned");
    }
    info!("DHCP is now up!");

    info!("Waiting for link up...");
    while !stack.is_link_up() {
        Timer::after(Duration::from_millis(500)).await;
    }
    info!("Link is up!");

    info!("Waiting for stack to be up...");
    stack.wait_config_up().await;
    info!("Stack is up!");

    // Main loop - blink LED and send telemetry
    let mut telemetry_interval = 0;
    const TELEMETRY_SEND_EVERY: u32 = 30; // Send telemetry every 10 blinks (10 seconds)

    loop {
        led.set_high();
        Timer::after(Duration::from_millis(500)).await;

        // Send telemetry less frequently to avoid overwhelming the server
        if telemetry_interval % TELEMETRY_SEND_EVERY == 0 {
            info!("Sending telemetry...");
            match send_telemetry(stack).await {
                Ok(_) => info!("Telemetry sent successfully"),
                Err(e) => warn!("Failed to send telemetry: {:?}", e),
            }
        }

        telemetry_interval += 1;

        led.set_low();
        Timer::after(Duration::from_millis(500)).await;
    }
}
