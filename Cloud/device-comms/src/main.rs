use device_comms::Application;


#[rocket::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = Application::build().await?;
    app.server.launch().await?;
    Ok(())
}
