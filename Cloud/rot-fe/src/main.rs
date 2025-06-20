// Import Yew framework prelude for web application development
use yew::prelude::*;
// Import custom components for navigation and header
use components::{Header, Navbar};
// Import view components for different application sections
use views::{TelemetryView, ConfigView};
// Import WASM-specific tracing configuration
use tracing_wasm::WASMLayerConfigBuilder;
// Import tracing subscriber prelude for logging setup
use tracing_subscriber::prelude::*;

// Module declarations - organize code into logical sections
mod components;  // UI components like header, navbar, charts
mod services;    // Business logic and API services
mod domain;      // Data models and domain logic
mod views;       // Main view components for different pages

/// Main application component that handles routing and layout
/// This component manages the current view state and renders the appropriate content
#[function_component(App)]
fn app() -> Html {
    // State to track which view is currently active (telemetry or config)
    // Default to "telemetry" view when the app starts
    let current_view = use_state(|| "telemetry".to_string());

    // Callback function to handle navigation clicks
    // This function updates the current view state when user clicks navigation items
    let on_nav_click = {
        let current_view = current_view.clone();
        Callback::from(move |view: String| {
            current_view.set(view);
        })
    };

    // Render the main application layout
    html! {
        <div>
            // Navigation bar component with click handler and current view indicator
            <Navbar on_nav_click={on_nav_click} current_view={(*current_view).clone()} />
            // Header component for branding/title
            <Header />
            // Conditional rendering based on current view selection
            {
                match (*current_view).as_str() {
                    "telemetry" => html! { <TelemetryView /> },
                    "config" => html! { <ConfigView /> },
                    _ => html! { <TelemetryView /> },  // Default fallback to telemetry view
                }
            }
        </div>
    }
}

/// Application entry point - initializes logging and starts the Yew application
fn main() {
    // Initialize tracing configuration for WASM environment
    // Set maximum log level to INFO for development and debugging
    let config = WASMLayerConfigBuilder::new()
        .set_max_level(tracing::Level::INFO)
        .build();
    
    // Set the WASM tracing layer as the global default for logging
    tracing_wasm::set_as_global_default_with_config(config);
    
    // Log application startup for debugging purposes
    tracing::info!("Application starting...");
    
    // Start the Yew application by rendering the App component
    yew::Renderer::<App>::new().render();
}