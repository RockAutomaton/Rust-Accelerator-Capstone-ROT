use defmt::*;
use embassy_net::Stack;
use embassy_time::{Duration, Timer};
use embedded_io_async::Write;
use heapless::String;
use serde_json_core::de::from_str;

use crate::config::device::{DeviceConfigItem, DeviceConfigResponse};
use crate::utils::config_store::set_device_config;

const CONFIG_URL_HOST: &str = env!("CONFIG_HOST");
const CONFIG_URL_PORT: u16 = 80;
const DEVICE_ID: &str = env!("DEVICE_ID");

#[embassy_executor::task]
pub async fn config_fetch_task(stack: Stack<'static>) {
    loop {
        match fetch_and_update_config(&stack).await {
            Ok(_) => info!("Config fetch and update succeeded"),
            Err(e) => warn!("Config fetch failed: {}", e),
        }
        Timer::after(Duration::from_secs(60)).await;
    }
}

async fn fetch_and_update_config(stack: &Stack<'_>) -> Result<(), &'static str> {
    let mut rx_buffer = [0; 1024];
    let mut tx_buffer = [0; 1024];
    let mut socket = embassy_net::tcp::TcpSocket::new(*stack, &mut rx_buffer, &mut tx_buffer);

    // DNS resolution
    let dns_socket = embassy_net::dns::DnsSocket::new(*stack);
    let addresses = dns_socket
        .query(CONFIG_URL_HOST, embassy_net::dns::DnsQueryType::A)
        .await
        .map_err(|_| "DNS resolution failed")?;
    let host_addr = *addresses
        .get(0)
        .ok_or("No IP addresses returned from DNS")?;

    // Connect to the server
    socket.set_timeout(Some(Duration::from_secs(10)));
    socket
        .connect(embassy_net::IpEndpoint::new(host_addr, CONFIG_URL_PORT))
        .await
        .map_err(|_| "Connection failed")?;

    // Build the path: /device-config/get/<DEVICE_ID>
    let mut path = String::<64>::new();
    let _ = core::fmt::write(&mut path, format_args!("/device-config/get/{}", DEVICE_ID));

    // Log the full URL being requested
    info!(
        "Fetching config from http://{}:{}{}",
        CONFIG_URL_HOST, CONFIG_URL_PORT, path
    );

    // Prepare HTTP GET request
    let mut request = String::<256>::new();
    let _ = core::fmt::write(
        &mut request,
        format_args!(
            "GET {} HTTP/1.1\r\nHost: {}\r\nConnection: close\r\nUser-Agent: RustEmbedded/1.0\r\n\r\n",
            path, CONFIG_URL_HOST
        ),
    );

    socket
        .write_all(request.as_bytes())
        .await
        .map_err(|_| "Write failed")?;

    // Read response
    let mut buf = [0; 1024];
    let n = socket.read(&mut buf).await.map_err(|_| "Read failed")?;
    let response = core::str::from_utf8(&buf[..n]).map_err(|_| "Invalid UTF-8")?;

    // Find start of JSON (skip HTTP headers)
    let json_start = response.find('[').ok_or("No JSON array in response")?;
    let json_str = &response[json_start..];

    // Parse JSON
    let (parsed, _): (DeviceConfigResponse, _) =
        from_str(json_str).map_err(|_| "JSON parse error")?;

    // Find config for this device
    let device_config = parsed
        .into_iter()
        .find(|item| item.device_id.as_str() == DEVICE_ID)
        .ok_or("Device config not found")?;

    set_device_config(device_config).await;
    Ok(())
}
