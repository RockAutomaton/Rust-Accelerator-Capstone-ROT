/// # Configuration Fetch Task
///
/// This module implements a task that periodically fetches device configuration
/// from the cloud backend. It allows remote configuration of device parameters.

use defmt::*;
use embassy_net::Stack;
use embassy_time::{Duration, Timer};
use embedded_io_async::Write;
use heapless::String;
use serde_json_core::de::from_str;

use crate::config::device::{DeviceConfigItem, DeviceConfigResponse};
use crate::utils::config_store::set_device_config;

// Configuration parameters from environment variables
// These are set at build time to avoid hardcoding sensitive information
/// The hostname of the configuration server
const CONFIG_URL_HOST: &str = env!("CONFIG_HOST");
/// The port of the configuration server (standard HTTP port)
const CONFIG_URL_PORT: u16 = 80;
/// The unique identifier for this device
const DEVICE_ID: &str = env!("DEVICE_ID");

/// Embassy task for periodically fetching device configuration from the cloud.
///
/// This task runs in a continuous loop, fetching configuration updates at 
/// regular intervals. It handles any errors that occur during the process
/// and retries on the next cycle.
///
/// # Parameters
/// * `stack` - Network stack for communication
///
/// # Note
/// This function never returns as it's designed to run for the entire
/// device lifecycle.
#[embassy_executor::task]
pub async fn config_fetch_task(stack: Stack<'static>) {
    // Main task loop - runs forever
    loop {
        // Attempt to fetch and update the device configuration
        match fetch_and_update_config(&stack).await {
            Ok(_) => info!("Config fetch and update succeeded"),
            Err(e) => warn!("Config fetch failed: {}", e),
        }
        
        // Wait 60 seconds before the next configuration check
        // This reduces network traffic while still allowing timely updates
        Timer::after(Duration::from_secs(60)).await;
    }
}

/// Fetches device configuration from the cloud server and updates local storage.
///
/// This function performs the following steps:
/// 1. Resolves the configuration server hostname using DNS
/// 2. Connects to the server
/// 3. Sends an HTTP GET request
/// 4. Receives and parses the response
/// 5. Updates the local configuration storage
///
/// # Parameters
/// * `stack` - Network stack for communication
///
/// # Returns
/// * `Ok(())` - If configuration was fetched and updated successfully
/// * `Err(&'static str)` - If any step fails, with an error message
async fn fetch_and_update_config(stack: &Stack<'_>) -> Result<(), &'static str> {
    // Create buffers for TCP socket (1KB each)
    let mut rx_buffer = [0; 1024];
    let mut tx_buffer = [0; 1024];
    
    // Create a new TCP socket using the network stack
    let mut socket = embassy_net::tcp::TcpSocket::new(*stack, &mut rx_buffer, &mut tx_buffer);

    // === DNS Resolution ===
    // Create a DNS socket to resolve the hostname to an IP address
    let dns_socket = embassy_net::dns::DnsSocket::new(*stack);
    
    // Query the DNS server for the host's IP address
    let addresses = dns_socket
        .query(CONFIG_URL_HOST, embassy_net::dns::DnsQueryType::A)
        .await
        .map_err(|_| "DNS resolution failed")?;
    
    // Get the first IP address from the result
    let host_addr = *addresses
        .get(0)
        .ok_or("No IP addresses returned from DNS")?;

    // === Connect to Server ===
    // Set connection timeout to 10 seconds to avoid hanging indefinitely
    socket.set_timeout(Some(Duration::from_secs(10)));
    
    // Connect to the configuration server
    socket
        .connect(embassy_net::IpEndpoint::new(host_addr, CONFIG_URL_PORT))
        .await
        .map_err(|_| "Connection failed")?;

    // === Prepare HTTP Request ===
    // Build the API path: /device-config/get/<DEVICE_ID>
    let mut path = String::<64>::new();
    let _ = core::fmt::write(&mut path, format_args!("/device-config/get/{}", DEVICE_ID));

    // Log the full URL being requested for debugging
    info!(
        "Fetching config from http://{}:{}{}",
        CONFIG_URL_HOST, CONFIG_URL_PORT, path
    );

    // Prepare HTTP GET request with proper headers
    // Using heapless String with fixed capacity for no-alloc environment
    let mut request = String::<256>::new();
    let _ = core::fmt::write(
        &mut request,
        format_args!(
            "GET {} HTTP/1.1\r\n\
             Host: {}\r\n\
             Connection: close\r\n\
             User-Agent: RustEmbedded/1.0\r\n\
             \r\n",
            path,           // API endpoint path
            CONFIG_URL_HOST // Host header value
        ),
    );

    // === Send HTTP Request ===
    // Write the request to the socket
    socket
        .write_all(request.as_bytes())
        .await
        .map_err(|_| "Write failed")?;

    // === Read HTTP Response ===
    // Create a buffer for the response (1KB)
    let mut buf = [0; 1024];
    
    // Read the response from the socket
    let n = socket.read(&mut buf).await.map_err(|_| "Read failed")?;
    
    // Convert the bytes to a UTF-8 string
    let response = core::str::from_utf8(&buf[..n]).map_err(|_| "Invalid UTF-8")?;

    // === Parse Response ===
    // Find start of JSON data (skip HTTP headers)
    // The API returns a JSON array that starts with '[' character
    let json_start = response.find('[').ok_or("No JSON array in response")?;
    let json_str = &response[json_start..];

    // Parse the JSON data into our DeviceConfigResponse type
    // Using no_std-compatible serde_json_core parser
    let (parsed, _): (DeviceConfigResponse, _) =
        from_str(json_str).map_err(|_| "JSON parse error")?;

    // Find the configuration specific to this device
    // The API returns configs for multiple devices, so we filter by device_id
    let device_config = parsed
        .into_iter()
        .find(|item| item.device_id.as_str() == DEVICE_ID)
        .ok_or("Device config not found")?;

    // === Store Configuration ===
    // Update the local configuration store with the new config
    set_device_config(device_config).await;
    
    // Return success
    Ok(())
}
