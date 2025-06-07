use yew::prelude::*;

#[function_component(Header)]
pub fn header() -> Html {
    html! {
        <header class="bg-black text-white py-10 shadow-lg border-b-4 border-green-500">
            <div class="max-w-3xl mx-auto text-center">
                <h1 class="text-5xl font-extrabold tracking-tight uppercase">
                    {"Rust of Things"}
                </h1>
                <p class="mt-4 text-lg font-medium text-green-400 tracking-wide">
                    {"A modern IoT platform powered by Rust."}
                </p>
            </div>
        </header>
    }
}
