use defmt::*;
use embassy_net::Stack;
use embassy_time::{Duration, Instant, Timer};
use embedded_io_async::Write;

use crate::config::TelemetryConfig;
use crate::drivers::TemperatureSensor;
use crate::error::TelemetryError;
use heapless::String;

pub struct TelemetryTaskConfig {
    pub interval_seconds: u32,
}

async fn send_telemetry(stack: &Stack<'_>, temperature: f32) -> Result<(), TelemetryError> {
    let mut rx_buffer = [0; 1024];
    let mut tx_buffer = [0; 1024];
    let mut socket = embassy_net::tcp::TcpSocket::new(*stack, &mut rx_buffer, &mut tx_buffer);

    // DNS resolution
    let dns_socket = embassy_net::dns::DnsSocket::new(*stack);

    info!("Resolving hostname: {}", TelemetryConfig::HOST);
    let addresses = match dns_socket
        .query(TelemetryConfig::HOST, embassy_net::dns::DnsQueryType::A)
        .await
    {
        Ok(addrs) => addrs,
        Err(_) => {
            warn!("DNS resolution failed");
            return Err(TelemetryError::DnsResolve);
        }
    };

    let host_addr = if let Some(addr) = addresses.get(0) {
        info!("Resolved {} to {}", TelemetryConfig::HOST, addr);
        *addr
    } else {
        warn!("No IP addresses returned from DNS");
        return Err(TelemetryError::DnsResolve);
    };

    // Connect to the server
    info!("Connecting to {}:{}", host_addr, TelemetryConfig::PORT);
    socket.set_timeout(Some(Duration::from_secs(10)));

    match socket
        .connect(embassy_net::IpEndpoint::new(
            host_addr,
            TelemetryConfig::PORT,
        ))
        .await
    {
        Ok(_) => info!("Connected successfully"),
        Err(e) => {
            warn!("Connection failed: {:?}", e);
            return Err(TelemetryError::Connect);
        }
    }

    // Prepare the telemetry data with actual temperature
    let mut telemetry_data = String::<256>::new();
    let _ = core::fmt::write(
        &mut telemetry_data,
        format_args!(
            "{{\"device_id\":\"1\",\"telemetry_data\":{{\"temperature\":\"{:.1}\",\"status\":\"active\"}}}}",
            temperature
        ),
    );

    // Prepare HTTP request
    let mut request = String::<512>::new();
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
            TelemetryConfig::PATH,
            TelemetryConfig::HOST,
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

    // Read response
    let mut buf = [0; 1024];
    match socket.read(&mut buf).await {
        Ok(n) => {
            let response = core::str::from_utf8(&buf[..n]).unwrap_or("Invalid UTF-8");
            info!("Response ({} bytes): {}", n, response);

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
    Timer::after(Duration::from_millis(100)).await;
    Ok(())
}

#[embassy_executor::task]
pub async fn telemetry_task(
    stack: Stack<'static>,
    config: TelemetryTaskConfig,
    mut temp_sensor: TemperatureSensor,
) -> ! {
    let mut telemetry_interval = 0;
    const TELEMETRY_SEND_EVERY: u32 = 30; // Send every 30 seconds

    loop {
        if telemetry_interval % TELEMETRY_SEND_EVERY == 0 {
            info!("Reading temperature and sending telemetry...");
            match temp_sensor.read_temperature().await {
                Ok(temperature) => match send_telemetry(&stack, temperature).await {
                    Ok(_) => info!("Telemetry sent successfully"),
                    Err(e) => warn!("Failed to send telemetry: {:?}", e),
                },
                Err(e) => warn!("Failed to read temperature: {:?}", e),
            }
        }

        telemetry_interval += 1;
        Timer::after(Duration::from_secs(1)).await;
    }
}
