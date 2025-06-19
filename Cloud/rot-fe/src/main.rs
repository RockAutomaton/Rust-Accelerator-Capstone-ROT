use yew::prelude::*;
use components::{Header, Navbar};
use views::{TelemetryView, ConfigView};
use tracing_wasm::WASMLayerConfigBuilder;
use tracing_subscriber::prelude::*;


mod components;
mod services;
mod domain;
mod views;

#[function_component]
fn App() -> Html {
    let current_view = use_state(|| "telemetry".to_string());

    let on_nav_click = {
        let current_view = current_view.clone();
        Callback::from(move |view: String| {
            current_view.set(view);
        })
    };

    html! {
        <div>
            <Navbar on_nav_click={on_nav_click} current_view={(*current_view).clone()} />
            <Header />
            {
                match (*current_view).as_str() {
                    "telemetry" => html! { <TelemetryView /> },
                    "config" => html! { <ConfigView /> },
                    _ => html! { <TelemetryView /> },
                }
            }
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