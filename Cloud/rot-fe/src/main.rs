use yew::prelude::*;
use components::{Header, Navbar, ApexChart, TelemetryView};


mod components;
mod services;
mod domain;

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
    yew::Renderer::<App>::new().render();
}