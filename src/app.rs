use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/ai-chat.css"/>

        // sets the document title
        <Title text="Welcome to AiChat"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                    <Route path="/*any" view=NotFound/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
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

/// 404 - Not Found
#[component]
fn NotFound() -> impl IntoView {
    // set an HTTP status code 404
    // this is feature gated because it can only be done during
    // initial server-side rendering
    // if you navigate to the 404 page subsequently, the status
    // code will not be set because there is not a new HTTP request
    // to the server
    #[cfg(feature = "ssr")]
    {
        // this can be done inline because it's synchronous
        // if it were async, we'd use a server function
        let resp = expect_context::<leptos_actix::ResponseOptions>();
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }

    view! {
        <div class="container flex items-center mt-20 flex-col">
            <h1 class="text-6xl font-bold text-gray-800">404</h1>
            <p class="text-2xl text-gray-600 mt-4">Page Not Found</p>
            <p class="text-gray-500 mt-2">"你似乎来到了没有知识存在的荒原"</p>
            <a
                href="/"
                class="mt-6 inline-block px-6 py-3 bg-blue-500 text-white font-semibold rounded-md hover:bg-blue-600"
            >
                "前往首页"
            </a>
        </div>
    }
}
