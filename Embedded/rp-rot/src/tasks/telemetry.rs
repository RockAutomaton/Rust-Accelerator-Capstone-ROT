/// # Telemetry Task
///
/// This module implements the telemetry task that periodically collects sensor data
/// and sends it to the cloud backend. It handles sensor reading, formatting the data
/// into JSON, and sending HTTP requests with error handling.

use defmt::*;
use embassy_net::Stack;
use embassy_time::{Duration, Instant, Timer};
use embedded_io_async::Write;

use crate::config::TelemetryConfig;
use crate::drivers::TemperatureSensor;
use crate::error::TelemetryError;
use heapless::String;

/// Configuration for the telemetry task.
///
/// This struct allows configuring the behavior of the telemetry task,
/// such as how often it should collect and send data.
pub struct TelemetryTaskConfig {
    /// Interval in seconds between telemetry data collections
    pub interval_seconds: u32,
}

/// Sends telemetry data to the cloud backend over HTTP.
///
/// This function performs the following steps:
/// 1. Creates a TCP socket
/// 2. Resolves the server hostname using DNS
/// 3. Connects to the server
/// 4. Formats the telemetry data as JSON
/// 5. Sends an HTTP POST request
/// 6. Processes the response
///
/// # Parameters
/// * `stack` - Network stack for TCP/IP communication
/// * `temperature` - Temperature reading in degrees Celsius
/// * `voltage` - Voltage reading in volts
///
/// # Returns
/// * `Ok(())` - If telemetry was sent successfully
/// * `Err(TelemetryError)` - If any step fails
async fn send_telemetry(
    stack: &Stack<'_>,
    temperature: f32,
    voltage: f32,
) -> Result<(), TelemetryError> {
    // Create buffers for TCP socket (1KB each)
    let mut rx_buffer = [0; 1024];
    let mut tx_buffer = [0; 1024];
    
    // Create a new TCP socket using the network stack
    let mut socket = embassy_net::tcp::TcpSocket::new(*stack, &mut rx_buffer, &mut tx_buffer);

    // === DNS Resolution ===
    // Create a DNS socket to resolve the hostname to an IP address
    let dns_socket = embassy_net::dns::DnsSocket::new(*stack);

    info!("Resolving hostname: {}", TelemetryConfig::HOST);
    // Query the DNS server for the host's IP address
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

    // Get the first IP address from the result (if any)
    let host_addr = if let Some(addr) = addresses.get(0) {
        info!("Resolved {} to {}", TelemetryConfig::HOST, addr);
        *addr
    } else {
        warn!("No IP addresses returned from DNS");
        return Err(TelemetryError::DnsResolve);
    };

    // === Connect to Server ===
    info!("Connecting to {}:{}", host_addr, TelemetryConfig::PORT);
    
    // Set connection timeout to 10 seconds to avoid hanging indefinitely
    socket.set_timeout(Some(Duration::from_secs(10)));

    // Attempt to connect to the server
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

    // === Format Telemetry Data as JSON ===
    // Create a fixed-size string for storing JSON data (up to 256 bytes)
    let mut telemetry_data = String::<256>::new();
    
    // Format telemetry data as JSON
    // Using heapless String with fixed capacity for no-alloc environment
    let _ = core::fmt::write(
        &mut telemetry_data,
        format_args!(
            // JSON structure with device ID, temperature, voltage, and status
            "{{\"device_id\":\"1\",\"telemetry_data\":{{\"temperature\":\"{:.1}\",\"voltage\":\"{:.2}\",\"status\":\"active\"}}}}",
            temperature, voltage
        ),
    );

    // === Prepare HTTP Request ===
    // Create a fixed-size string for storing the HTTP request (up to 512 bytes)
    let mut request = String::<512>::new();
    
    // Format the complete HTTP request with headers and body
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
            TelemetryConfig::PATH,     // API endpoint path
            TelemetryConfig::HOST,     // Host header value
            telemetry_data.len(),      // Content length
            telemetry_data             // Request body (JSON)
        ),
    );

    info!("Sending HTTP request ({} bytes)", request.len());

    // === Send HTTP Request ===
    // Write the request to the socket
    match socket.write_all(request.as_bytes()).await {
        Ok(_) => info!("Request sent successfully"),
        Err(e) => {
            warn!("Failed to send request: {:?}", e);
            return Err(TelemetryError::Write);
        }
    }

    // === Read HTTP Response ===
    // Create a buffer for the response (1KB)
    let mut buf = [0; 1024];
    
    // Read the response from the socket
    match socket.read(&mut buf).await {
        Ok(n) => {
            // Convert the bytes to a UTF-8 string, using a fallback if invalid
            let response = core::str::from_utf8(&buf[..n]).unwrap_or("Invalid UTF-8");
            info!("Response ({} bytes): {}", n, response);

            // Check if the response indicates success (HTTP 200 OK)
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

    // === Clean Up ===
    // Close the socket to free resources
    socket.close();
    
    // Wait a short time to ensure the connection is properly closed
    Timer::after(Duration::from_millis(100)).await;
    
    // Return success
    Ok(())
}

/// Embassy task for periodically collecting and sending telemetry data.
///
/// This long-running task performs the following operations on a regular schedule:
/// 1. Reads temperature and voltage from sensors
/// 2. Formats the data
/// 3. Sends it to the cloud backend
/// 4. Handles any errors that occur
///
/// # Parameters
/// * `stack` - Network stack for communication
/// * `config` - Configuration for the telemetry task
/// * `temp_sensor` - Temperature sensor driver
///
/// # Note
/// This function never returns (-> !) as it's designed to run for the entire
/// device lifecycle.
#[embassy_executor::task]
pub async fn telemetry_task(
    stack: Stack<'static>,
    config: TelemetryTaskConfig,
    mut temp_sensor: TemperatureSensor,
) -> ! {
    // Counter for tracking intervals
    let mut telemetry_interval = 0;
    
    // How often to send telemetry data (in seconds)
    const TELEMETRY_SEND_EVERY: u32 = 30;

    // Main task loop - runs forever
    loop {
        // Check if it's time to send telemetry
        if telemetry_interval % TELEMETRY_SEND_EVERY == 0 {
            info!("Reading sensors and sending telemetry...");
            
            // Read temperature and voltage in parallel
            match (
                temp_sensor.read_temperature().await,
                temp_sensor.read_voltage().await,
            ) {
                // If both readings are successful
                (Ok(temperature), Ok(voltage)) => {
                    // Send the telemetry data to the server
                    match send_telemetry(&stack, temperature, voltage).await {
                        Ok(_) => info!("Telemetry sent successfully"),
                        Err(e) => warn!("Failed to send telemetry: {:?}", e),
                    }
                }
                // Handle sensor reading errors
                (Err(e), _) => warn!("Failed to read temperature: {:?}", e),
                (_, Err(e)) => warn!("Failed to read voltage: {:?}", e),
            }
        }

        // Increment the interval counter
        telemetry_interval += 1;
        
        // Wait 1 second before the next iteration
        Timer::after(Duration::from_secs(1)).await;
    }
}
