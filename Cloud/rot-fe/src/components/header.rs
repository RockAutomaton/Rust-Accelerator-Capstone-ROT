// Import Yew framework prelude for component development
use yew::prelude::*;

/// Header component that displays the application title and tagline
/// This component provides branding and visual identity for the application
#[function_component(Header)]
pub fn header() -> Html {
    // Render the header with dark theme and green accent colors
    html! {
        <header class="bg-black text-white py-10 shadow-lg border-b-4 border-green-500">
            <div class="max-w-3xl mx-auto text-center">
                // Main application title with large, bold typography
                <h1 class="text-5xl font-extrabold tracking-tight uppercase">
                    {"Rust of Things"}
                </h1>
                // Subtitle describing the platform with green accent color
                <p class="mt-4 text-lg font-medium text-green-400 tracking-wide">
                    {"A modern IoT platform powered by Rust."}
                </p>
            </div>
        </header>
    }
}
