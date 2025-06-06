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
                    {"A modern IoT platform powered by Rust and the Cloud"}
                </p>
                <div class="mt-6 flex justify-center gap-4">
                    <button class="bg-green-500 hover:bg-green-600 text-black font-bold py-2 px-6 rounded transition">
                        {"Get Started"}
                    </button>
                    <button class="bg-white hover:bg-green-100 text-black font-bold py-2 px-6 rounded border border-green-500 transition">
                        {"Learn More"}
                    </button>
                </div>
            </div>
        </header>
    }
}
