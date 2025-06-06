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
                    <a href="#" class="text-white hover:text-green-400 font-medium transition">{"Home"}</a>
                    <a href="#" class="text-white hover:text-green-400 font-medium transition">{"Features"}</a>
                    <a href="#" class="text-white hover:text-green-400 font-medium transition">{"Docs"}</a>
                    <a href="#" class="text-white hover:text-green-400 font-medium transition">{"Contact"}</a>
                </div>
                // Call to Action
                <div class="hidden md:block">
                    <a href="#" class="bg-green-500 hover:bg-green-600 text-black font-bold py-2 px-5 rounded transition">
                        {"Get Started"}
                    </a>
                </div>
                // Mobile menu icon (optional, not functional here)
                <div class="md:hidden">
                    <button class="text-green-500 focus:outline-none">
                        <svg class="w-7 h-7" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M4 6h16M4 12h16M4 18h16"/>
                        </svg>
                    </button>
                </div>
            </div>
        </nav>
    }
}
