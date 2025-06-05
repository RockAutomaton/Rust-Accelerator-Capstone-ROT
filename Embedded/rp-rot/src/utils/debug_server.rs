use defmt::*;
use embassy_net::{IpAddress, IpEndpoint, Stack};
use embassy_time::{Duration, Timer};
use embedded_io_async::Write;
use heapless::String;

use crate::config::TelemetryConfig;

const LOCAL_DEBUG_PORT: u16 = 8000;

pub async fn post_to_debug_server(stack: &Stack<'_>, log_data: &str) -> Result<(), &'static str> {
    // Try to send to local debug server if configured
    if let Some(debug_server) = option_env!("DEBUG_SERVER") {
        if let Err(e) = send_to_local_debug_server(stack, debug_server, log_data).await {
            warn!("Failed to send to local debug server: {}", e);
        }
    }

    // Then send to Azure
    send_to_azure(stack, log_data).await
}

async fn send_to_local_debug_server(
    stack: &Stack<'_>,
    debug_server: &str,
    log_data: &str,
) -> Result<(), &'static str> {
    let mut rx_buffer = [0; 1024];
    let mut tx_buffer = [0; 1024];
    let mut socket = embassy_net::tcp::TcpSocket::new(*stack, &mut rx_buffer, &mut tx_buffer);

    // DNS resolution for local debug server
    let dns_socket = embassy_net::dns::DnsSocket::new(*stack);

    info!("Resolving local debug server hostname: {}", debug_server);
    let addresses = match dns_socket
        .query(debug_server, embassy_net::dns::DnsQueryType::A)
        .await
    {
        Ok(addrs) => addrs,
        Err(_) => {
            warn!("Local debug server DNS resolution failed");
            return Err("DNS resolution failed");
        }
    };

    let host_addr = if let Some(addr) = addresses.get(0) {
        info!("Resolved {} to {}", debug_server, addr);
        *addr
    } else {
        warn!("No IP addresses returned from local debug server DNS");
        return Err("No IP addresses returned from DNS");
    };

    // Connect to the local server
    info!(
        "Connecting to local debug server {}:{}",
        host_addr, LOCAL_DEBUG_PORT
    );
    socket.set_timeout(Some(Duration::from_secs(5)));

    match socket
        .connect(embassy_net::IpEndpoint::new(host_addr, LOCAL_DEBUG_PORT))
        .await
    {
        Ok(_) => info!("Connected to local debug server successfully"),
        Err(e) => {
            warn!("Local debug server connection failed: {:?}", e);
            return Err("Connection failed");
        }
    }

    // Prepare HTTP request
    let mut request = String::<512>::new();
    let _ = core::fmt::write(
        &mut request,
        format_args!(
            "POST /log HTTP/1.1\r\n\
             Host: {}\r\n\
             Content-Type: application/json\r\n\
             Content-Length: {}\r\n\
             Connection: close\r\n\
             User-Agent: RustEmbedded/1.0\r\n\
             \r\n\
             {}",
            debug_server,
            log_data.len(),
            log_data
        ),
    );

    info!("Sending local debug log request ({} bytes)", request.len());

    // Send the request
    match socket.write_all(request.as_bytes()).await {
        Ok(_) => info!("Local debug log request sent successfully"),
        Err(e) => {
            warn!("Failed to send local debug log request: {:?}", e);
            return Err("Write failed");
        }
    }

    // Read response
    let mut buf = [0; 1024];
    match socket.read(&mut buf).await {
        Ok(n) => {
            let response = core::str::from_utf8(&buf[..n]).unwrap_or("Invalid UTF-8");
            info!("Local debug server response ({} bytes): {}", n, response);

            if response.contains("HTTP/1.1 200") || response.contains("HTTP/1.0 200") {
                info!("Local debug log accepted by server");
                Ok(())
            } else {
                warn!("Local debug server returned non-200 status");
                Err("Server returned non-200 status")
            }
        }
        Err(e) => {
            warn!("Failed to read local debug server response: {:?}", e);
            Err("Read failed")
        }
    }
}

async fn send_to_azure(stack: &Stack<'_>, log_data: &str) -> Result<(), &'static str> {
    let mut rx_buffer = [0; 1024];
    let mut tx_buffer = [0; 1024];
    let mut socket = embassy_net::tcp::TcpSocket::new(*stack, &mut rx_buffer, &mut tx_buffer);

    // DNS resolution
    let dns_socket = embassy_net::dns::DnsSocket::new(*stack);

    info!("Resolving Azure hostname: {}", TelemetryConfig::HOST);
    let addresses = match dns_socket
        .query(TelemetryConfig::HOST, embassy_net::dns::DnsQueryType::A)
        .await
    {
        Ok(addrs) => addrs,
        Err(_) => {
            warn!("Azure DNS resolution failed");
            return Err("DNS resolution failed");
        }
    };

    let host_addr = if let Some(addr) = addresses.get(0) {
        info!("Resolved {} to {}", TelemetryConfig::HOST, addr);
        *addr
    } else {
        warn!("No IP addresses returned from Azure DNS");
        return Err("No IP addresses returned from DNS");
    };

    // Connect to Azure
    info!(
        "Connecting to Azure {}:{}",
        host_addr,
        TelemetryConfig::PORT
    );
    socket.set_timeout(Some(Duration::from_secs(10)));

    match socket
        .connect(embassy_net::IpEndpoint::new(
            host_addr,
            TelemetryConfig::PORT,
        ))
        .await
    {
        Ok(_) => info!("Connected to Azure successfully"),
        Err(e) => {
            warn!("Azure connection failed: {:?}", e);
            return Err("Connection failed");
        }
    }

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
            log_data.len(),
            log_data
        ),
    );

    info!("Sending Azure debug log request ({} bytes)", request.len());

    // Send the request
    match socket.write_all(request.as_bytes()).await {
        Ok(_) => info!("Azure debug log request sent successfully"),
        Err(e) => {
            warn!("Failed to send Azure debug log request: {:?}", e);
            return Err("Write failed");
        }
    }

    // Read response
    let mut buf = [0; 1024];
    match socket.read(&mut buf).await {
        Ok(n) => {
            let response = core::str::from_utf8(&buf[..n]).unwrap_or("Invalid UTF-8");
            info!("Azure response ({} bytes): {}", n, response);

            if response.contains("HTTP/1.1 200") || response.contains("HTTP/1.0 200") {
                info!("Azure debug log accepted");
                Ok(())
            } else {
                warn!("Azure returned non-200 status");
                Err("Server returned non-200 status")
            }
        }
        Err(e) => {
            warn!("Failed to read Azure response: {:?}", e);
            Err("Read failed")
        }
    }
}
