use defmt::{error, info};
use embassy_net::{Config, Stack, StackResources};
use embassy_rp::clocks::RoscRng;
use embassy_time::{Duration, Timer};
use rand_core::RngCore;
use static_cell::StaticCell;

pub struct NetworkStack;

impl NetworkStack {
    pub fn init(
        net_device: cyw43::NetDriver<'static>,
    ) -> (
        Stack<'static>,
        embassy_net::Runner<'static, cyw43::NetDriver<'static>>,
    ) {
        let config = Config::dhcpv4(Default::default());
        let mut rng = RoscRng;
        let seed = rng.next_u64();

        static RESOURCES: StaticCell<StackResources<5>> = StaticCell::new();
        embassy_net::new(
            net_device,
            config,
            RESOURCES.init(StackResources::new()),
            seed,
        )
    }

    pub async fn setup_and_wait(stack: &Stack<'static>) -> Result<(), ()> {
        // Wait for DHCP with timeout
        info!("Waiting for DHCP...");
        let mut dhcp_timeout = 30; // 30 seconds timeout
        while !stack.is_config_up() && dhcp_timeout > 0 {
            Timer::after(Duration::from_secs(1)).await;
            dhcp_timeout -= 1;
        }

        if !stack.is_config_up() {
            error!("DHCP failed - no IP address assigned");
            return Err(());
        }
        info!("DHCP is now up!");

        // Wait for link up
        info!("Waiting for link up...");
        while !stack.is_link_up() {
            Timer::after(Duration::from_millis(500)).await;
        }
        info!("Link is up!");

        // Wait for stack to be up
        info!("Waiting for stack to be up...");
        stack.wait_config_up().await;
        info!("Stack is up!");

        Ok(())
    }

    pub fn get_config_info(stack: &Stack<'static>) -> NetworkInfo {
        NetworkInfo {
            is_config_up: stack.is_config_up(),
            is_link_up: stack.is_link_up(),
        }
    }
}

#[derive(Debug, defmt::Format)]
pub struct NetworkInfo {
    pub is_config_up: bool,
    pub is_link_up: bool,
}

impl NetworkInfo {
    pub fn log_status(&self) {
        info!(
            "Network status: config_up={}, link_up={}",
            self.is_config_up, self.is_link_up
        );
    }
}
