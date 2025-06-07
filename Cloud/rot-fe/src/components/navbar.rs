use yew::prelude::*;

#[function_component(Navbar)]
pub fn navbar() -> Html {
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
                    <a href="#" class="text-white hover:text-green-400 font-medium transition">{"Device Monitoring"}</a>
                    <a href="#" class="text-white hover:text-green-400 font-medium transition">{"Device Management (Coming Soon)"}</a>
                </div>
            </div>
        </nav>
    }
}
