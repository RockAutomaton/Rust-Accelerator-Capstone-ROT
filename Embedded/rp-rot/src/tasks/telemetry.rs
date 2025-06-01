use crate::config::TelemetryConfig;
use crate::error::TelemetryError;
use defmt::*;
use embassy_net::{
    dns::{DnsQueryType, DnsSocket},
    tcp::TcpSocket,
    IpEndpoint, Stack,
};
use embassy_time::{Duration, Timer};
use embedded_io_async::{Read, Write};
use heapless::String;

#[derive(Debug, Clone)]
pub struct TelemetryTaskConfig {
    pub interval_seconds: u64,
    pub max_retry_attempts: u32,
    pub retry_delay_seconds: u64,
    pub enable_backoff: bool,
    pub max_backoff_seconds: u64,
}

async fn send_telemetry(stack: &Stack<'_>) -> Result<(), TelemetryError> {
    let mut rx_buffer = [0; 1024];
    let mut tx_buffer = [0; 1024];
    let mut socket = TcpSocket::new(*stack, &mut rx_buffer, &mut tx_buffer);

    // DNS resolution
    let dns_socket = DnsSocket::new(*stack);

    let config = TelemetryConfig::default();
    info!("Resolving hostname: {}", config.host);
    let addresses = match dns_socket.query(config.host, DnsQueryType::A).await {
        Ok(addrs) => addrs,
        Err(_) => {
            warn!("DNS resolution failed");
            return Err(TelemetryError::DnsResolve);
        }
    };

    let host_addr = if let Some(addr) = addresses.get(0) {
        info!("Resolved {} to {}", config.host, addr);
        *addr
    } else {
        warn!("No IP addresses returned from DNS");
        return Err(TelemetryError::DnsResolve);
    };

    // Connect to the server
    info!("Connecting to {}:{}", host_addr, config.port);
    socket.set_timeout(Some(Duration::from_secs(10)));

    match socket
        .connect(IpEndpoint::new(host_addr, config.port))
        .await
    {
        Ok(_) => info!("Connected successfully"),
        Err(e) => {
            warn!("Connection failed: {:?}", e);
            return Err(TelemetryError::Connect);
        }
    }

    let telemetry_data = "{\"device_id\":\"1\",\"telemetry_data\":{\"temperature\":\"25.5\",\"humidity\":\"60.0\",\"status\":\"active\"}}";

    let mut request = String::<512>::new();
    if core::fmt::write(
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
            config.path,
            config.host,
            telemetry_data.len(),
            telemetry_data
        ),
    )
    .is_err()
    {
        warn!("Failed to format HTTP request");
        return Err(TelemetryError::Write);
    }

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
                return Err(TelemetryError::InvalidResponse);
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

#[embassy_executor::task]
pub async fn telemetry_with_retry_task(stack: Stack<'static>, config: TelemetryTaskConfig) -> ! {
    info!("Starting telemetry task with retry logic");
    let mut retry_count = 0;
    let mut backoff_seconds = config.retry_delay_seconds;

    // Wait for network to be ready
    while !stack.is_link_up() || !stack.is_config_up() {
        info!("Waiting for network to be ready...");
        Timer::after(Duration::from_secs(1)).await;
    }

    loop {
        match send_telemetry(&stack).await {
            Ok(_) => {
                info!("Telemetry sent successfully");
                retry_count = 0;
                backoff_seconds = config.retry_delay_seconds;
                Timer::after(Duration::from_secs(config.interval_seconds)).await;
            }
            Err(e) => {
                warn!("Telemetry failed: {:?}", e);
                retry_count += 1;

                if retry_count >= config.max_retry_attempts {
                    error!("Max retry attempts reached");
                    if config.enable_backoff {
                        backoff_seconds = (backoff_seconds * 2).min(config.max_backoff_seconds);
                    }
                    retry_count = 0;
                }

                info!("Retrying in {} seconds...", backoff_seconds);
                Timer::after(Duration::from_secs(backoff_seconds)).await;
            }
        }
    }
}
