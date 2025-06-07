use yew::prelude::*;
use components::{Header, Navbar};
use views::TelemetryView;


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
    yew::Renderer::<App>::new().render();
}