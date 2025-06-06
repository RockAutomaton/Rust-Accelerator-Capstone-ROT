use yew::prelude::*;
use components::{Header, Navbar};


mod components;

#[function_component]
fn App() -> Html {


    html! {
        <div>
            <Navbar />
            <Header />
        </div>
        
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}