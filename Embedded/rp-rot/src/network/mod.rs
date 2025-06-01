use defmt::{error, info};
use embassy_executor::Spawner;
use embassy_net::{Config, Stack, StackResources};
use embassy_rp::clocks::RoscRng;
use embassy_time::{Duration, Timer};
use rand_core::RngCore;
use static_cell::StaticCell;

pub struct NetworkStack;

impl NetworkStack {
    pub async fn init(
        net_device: cyw43::NetDriver<'static>,
        seed: u64,
        spawner: &Spawner,
    ) -> Stack<'static> {
        info!("Initializing network stack...");
        let config = Config::dhcpv4(Default::default());

        static RESOURCES: StaticCell<StackResources<5>> = StaticCell::new();
        let (stack, runner) = embassy_net::new(
            net_device,
            config,
            RESOURCES.init(StackResources::new()),
            seed,
        );

        spawner.spawn(network_task(runner)).unwrap();
        info!("Network stack initialized");
        stack
    }

    pub async fn setup_and_wait(stack: &Stack<'static>) -> Result<(), ()> {
        // Wait for DHCP with timeout
        info!("Waiting for DHCP...");
        let mut dhcp_timeout = 30; // 30 seconds timeout
        while !stack.is_config_up() && dhcp_timeout > 0 {
            Timer::after(Duration::from_secs(1)).await;
            dhcp_timeout -= 1;
            info!("Waiting for DHCP... {} seconds remaining", dhcp_timeout);
        }

        if !stack.is_config_up() {
            error!("DHCP failed - no IP address assigned");
            return Err(());
        }
        info!("DHCP is now up!");

        // Wait for link up with timeout
        info!("Waiting for link up...");
        let mut link_timeout = 10; // 10 seconds timeout
        while !stack.is_link_up() && link_timeout > 0 {
            Timer::after(Duration::from_millis(500)).await;
            link_timeout -= 1;
            info!("Waiting for link up... {} seconds remaining", link_timeout);
        }

        if !stack.is_link_up() {
            error!("Link failed to come up");
            return Err(());
        }
        info!("Link is up!");

        // Wait for stack to be up with timeout
        info!("Waiting for stack to be up...");
        let mut stack_timeout = 10; // 10 seconds timeout
        while !stack.is_config_up() && stack_timeout > 0 {
            Timer::after(Duration::from_secs(1)).await;
            stack_timeout -= 1;
            info!("Waiting for stack... {} seconds remaining", stack_timeout);
        }

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

#[embassy_executor::task]
async fn network_task(mut runner: embassy_net::Runner<'static, cyw43::NetDriver<'static>>) -> ! {
    info!("Starting network runner task");
    runner.run().await;
    info!("This should never be reached");
    loop {}
}
