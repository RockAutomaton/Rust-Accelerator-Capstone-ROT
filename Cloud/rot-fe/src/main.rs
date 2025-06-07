use yew::prelude::*;
use components::{Header, Navbar};
use views::TelemetryView;
use tracing_wasm::WASMLayerConfigBuilder;
use tracing_subscriber::prelude::*;


mod components;
mod services;
mod domain;
mod views;

#[function_component]
fn App() -> Html {


    html! {
        <div>
            <Navbar />
            <Header />
            // <ApexChart />
            <TelemetryView />
        </div>
        
    }
}

fn main() {
    // Initialize tracing
    let config = WASMLayerConfigBuilder::new()
        .set_max_level(tracing::Level::INFO)
        .build();
    
    tracing_wasm::set_as_global_default_with_config(config);
    
    // Log that the app is starting
    tracing::info!("Application starting...");
    
    yew::Renderer::<App>::new().render();
}