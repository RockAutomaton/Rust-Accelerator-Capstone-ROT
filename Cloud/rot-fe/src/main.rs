use yew::prelude::*;
use components::{Header, Navbar, ApexChart};


mod components;

#[function_component]
fn App() -> Html {


    html! {
        <div>
            <Navbar />
            <Header />
            <ApexChart />
        </div>
        
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}