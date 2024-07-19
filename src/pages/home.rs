use leptos::*;

/// Renders the home page of your application.
#[component]
pub fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(0);
    let on_click = move |_| {
        logging::log!("==> original count = {}", count());
        set_count.update(|count| *count += 1)
    };

    view! {
        <div class="container flex items-center mt-20 flex-col">
            <h1 class="text-4xl font-bold text-cyan-700 mb-4">"Welcome to AiChat!"</h1>
            <div class="">
                <span class="text-red-400">"Value: " {count} "!"</span>
            </div>
            <div class="flex gap-2 items-center mt-2">
                <button
                    on:click=on_click
                    class="hover:bg-green-400 rounded-md bg-green-500 text-white text-sm font-medium pl-2 pr-3 py-2 shadow-sm"
                >
                    "Click Me"
                </button>
            </div>
        </div>
    }
}
