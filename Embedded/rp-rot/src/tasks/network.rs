/// # Network Stack Task
///
/// This module defines a task that runs the TCP/IP network stack in the background.
/// It handles all network processing, including TCP/IP protocols and DHCP.

use defmt::*;

/// Embassy task for running the TCP/IP network stack.
///
/// This task handles all network protocol processing including:
/// - TCP/IP packet processing
/// - DHCP client for IP address acquisition
/// - DNS resolution
/// - Socket management
///
/// # Parameters
/// * `runner` - Network stack runner instance
///
/// # Note
/// This function never returns as it's designed to run for the entire
/// device lifecycle. It must be spawned before any network operations are attempted.
#[embassy_executor::task]
pub async fn network_task(
    mut runner: embassy_net::Runner<'static, cyw43::NetDriver<'static>>,
) -> ! {
    info!("Starting network runner task");
    
    // This call blocks indefinitely, processing network events
    runner.run().await;
    
    // This code should never be reached under normal operation
    #[allow(unreachable_code)]
    loop {
        // Safety loop in case the runner exits unexpectedly
    }
}
