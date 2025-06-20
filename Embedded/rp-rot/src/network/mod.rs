/// # Network Stack Manager
///
/// This module provides functionality for initializing and managing the TCP/IP
/// network stack. It handles DHCP configuration, link setup, and provides 
/// information about the network status.

use defmt::{error, info};
use embassy_executor::Spawner;
use embassy_net::{Config, Stack, StackResources};
use embassy_rp::clocks::RoscRng;
use embassy_time::{Duration, Timer};
use rand_core::RngCore;
use static_cell::StaticCell;

/// Provides methods for network stack management.
///
/// This struct contains static methods to initialize, configure, and monitor
/// the network stack.
pub struct NetworkStack;

impl NetworkStack {
    /// Initializes the network stack and spawns the network task.
    ///
    /// This function sets up the TCP/IP stack with DHCP configuration and
    /// spawns a background task to handle network operations.
    ///
    /// # Parameters
    /// * `net_device` - Network device driver (WiFi driver)
    /// * `seed` - Random seed for the network stack
    /// * `spawner` - Task spawner for creating the network task
    ///
    /// # Returns
    /// * `Stack<'static>` - The initialized network stack
    pub async fn init(
        net_device: cyw43::NetDriver<'static>,
        seed: u64,
        spawner: &Spawner,
    ) -> Stack<'static> {
        info!("Initializing network stack...");
        
        // Configure the network stack to use DHCP for IP address assignment
        let config = Config::dhcpv4(Default::default());

        // Create static storage for network stack resources
        // The '5' here defines the maximum number of sockets that can be open simultaneously
        static RESOURCES: StaticCell<StackResources<5>> = StaticCell::new();
        
        // Initialize the network stack
        let (stack, runner) = embassy_net::new(
            net_device,
            config,
            RESOURCES.init(StackResources::new()),
            seed,
        );

        // Spawn a task to run the network stack in the background
        spawner.spawn(network_task(runner)).unwrap();
        
        info!("Network stack initialized");
        stack
    }

    /// Waits for the network stack to be fully configured and ready.
    ///
    /// This function performs the following steps with timeouts:
    /// 1. Waits for DHCP to assign an IP address
    /// 2. Waits for the network link to be established
    /// 3. Waits for the network stack to be fully configured
    ///
    /// # Parameters
    /// * `stack` - Reference to the network stack
    ///
    /// # Returns
    /// * `Ok(())` - If the network is successfully configured
    /// * `Err(())` - If any step fails or times out
    pub async fn setup_and_wait(stack: &Stack<'static>) -> Result<(), ()> {
        // === Wait for DHCP with timeout ===
        info!("Waiting for DHCP...");
        let mut dhcp_timeout = 30; // 30 seconds timeout
        
        // Poll DHCP status until it's up or timeout expires
        while !stack.is_config_up() && dhcp_timeout > 0 {
            Timer::after(Duration::from_secs(1)).await;
            dhcp_timeout -= 1;
            info!("Waiting for DHCP... {} seconds remaining", dhcp_timeout);
        }

        // Check if DHCP succeeded
        if !stack.is_config_up() {
            error!("DHCP failed - no IP address assigned");
            return Err(());
        }
        info!("DHCP is now up!");

        // === Wait for link up with timeout ===
        info!("Waiting for link up...");
        let mut link_timeout = 10; // 10 seconds timeout
        
        // Poll link status until it's up or timeout expires
        while !stack.is_link_up() && link_timeout > 0 {
            Timer::after(Duration::from_millis(500)).await;
            link_timeout -= 1;
            info!("Waiting for link up... {} seconds remaining", link_timeout);
        }

        // Check if link came up successfully
        if !stack.is_link_up() {
            error!("Link failed to come up");
            return Err(());
        }
        info!("Link is up!");

        // === Wait for stack to be fully configured ===
        info!("Waiting for stack to be up...");
        let mut stack_timeout = 10; // 10 seconds timeout
        
        // Poll stack configuration status until it's up or timeout expires
        while !stack.is_config_up() && stack_timeout > 0 {
            Timer::after(Duration::from_secs(1)).await;
            stack_timeout -= 1;
            info!("Waiting for stack... {} seconds remaining", stack_timeout);
        }

        // Check if stack is fully configured
        if !stack.is_config_up() {
            error!("Stack failed to come up");
            return Err(());
        }
        info!("Stack is up!");

        // Additional wait to ensure everything is stable
        Timer::after(Duration::from_secs(2)).await;
        info!("Network stack is ready!");

        Ok(())
    }

    /// Gets the current network configuration status.
    ///
    /// This function retrieves the current status of the network stack,
    /// including whether DHCP configuration is up and the link is established.
    ///
    /// # Parameters
    /// * `stack` - Reference to the network stack
    ///
    /// # Returns
    /// * `NetworkInfo` - Structure containing network status information
    pub fn get_config_info(stack: &Stack<'static>) -> NetworkInfo {
        NetworkInfo {
            is_config_up: stack.is_config_up(),
            is_link_up: stack.is_link_up(),
        }
    }
}

/// Contains information about the current network status.
///
/// This struct holds various flags indicating the state of the network
/// connection and configuration.
#[derive(Debug, defmt::Format)]
pub struct NetworkInfo {
    /// Whether the network configuration (DHCP) is up
    pub is_config_up: bool,
    
    /// Whether the network link is established
    pub is_link_up: bool,
}

impl NetworkInfo {
    /// Logs the current network status to the debug output.
    ///
    /// This function is useful for debugging network issues.
    pub fn log_status(&self) {
        info!(
            "Network status: config_up={}, link_up={}",
            self.is_config_up, self.is_link_up
        );
    }
}

/// Embassy task that runs the network stack.
///
/// This task handles all the network processing in the background,
/// including TCP/IP protocols, DHCP, and other network operations.
///
/// # Parameters
/// * `runner` - Network stack runner instance
///
/// # Note
/// This function never returns as it's designed to run for the entire
/// device lifecycle.
#[embassy_executor::task]
async fn network_task(mut runner: embassy_net::Runner<'static, cyw43::NetDriver<'static>>) -> ! {
    info!("Starting network runner task");
    
    // This call blocks indefinitely, processing network events
    runner.run().await;
    
    // This code should never be reached under normal operation
    info!("This should never be reached");
    loop {}
}
