// Import Yew framework prelude for component development
use yew::prelude::*;

/// Properties for the Navbar component
/// - on_nav_click: Callback to handle navigation button clicks
/// - current_view: String indicating which view is currently active
#[derive(Properties, PartialEq)]
pub struct NavbarProps {
    pub on_nav_click: Callback<String>,
    pub current_view: String,
}

/// Navbar component for application navigation
/// Renders navigation buttons and highlights the active view
#[function_component(Navbar)]
pub fn navbar(props: &NavbarProps) -> Html {
    // Callback for the "Device Monitoring" button
    // Emits "telemetry" when clicked
    let on_telemetry_click = {
        let on_nav_click = props.on_nav_click.clone();
        Callback::from(move |_| {
            on_nav_click.emit("telemetry".to_string());
        })
    };

    // Callback for the "Device Configuration" button
    // Emits "config" when clicked
    let on_config_click = {
        let on_nav_click = props.on_nav_click.clone();
        Callback::from(move |_| {
            on_nav_click.emit("config".to_string());
        })
    };

    // Render the navigation bar with branding and navigation buttons
    html! {
        <nav class="bg-black border-b-2 border-green-500 px-6 py-4">
            <div class="max-w-6xl mx-auto flex items-center justify-between">
                // Logo or Brand
                <div class="flex items-center gap-2">
                    <span class="text-green-500 font-extrabold text-2xl tracking-widest">{"ROT"}</span>
                    <span class="text-white font-bold text-lg">{"Rust of Things"}</span>
                </div>
                // Navigation Links
                <div class="hidden md:flex gap-8">
                    // Device Monitoring button, highlighted if active
                    <button
                        onclick={on_telemetry_click}
                        class={format!(
                            "font-medium transition {}",
                            if props.current_view == "telemetry" {
                                "text-green-400"
                            } else {
                                "text-white hover:text-green-400"
                            }
                        )}
                    >
                        {"Device Monitoring"}
                    </button>
                    // Device Configuration button, highlighted if active
                    <button
                        onclick={on_config_click}
                        class={format!(
                            "font-medium transition {}",
                            if props.current_view == "config" {
                                "text-green-400"
                            } else {
                                "text-white hover:text-green-400"
                            }
                        )}
                    >
                        {"Device Configuration"}
                    </button>
                </div>
            </div>
        </nav>
    }
}
