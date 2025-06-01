use defmt::*;

#[embassy_executor::task]
pub async fn network_task(
    mut runner: embassy_net::Runner<'static, cyw43::NetDriver<'static>>,
) -> ! {
    info!("Starting network stack background task");
    runner.run().await
}
