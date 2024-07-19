use leptos::*;

/// 404 - Not Found
#[component]
pub fn NotFound() -> impl IntoView {
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
            <h1 class="text-6xl font-bold text-cyan-700">404</h1>
            <p class="text-2xl text-cyan-700 mt-4">Page Not Found</p>
            <a
                href="/"
                class="mt-6 inline-block px-6 py-3 bg-blue-500 text-white font-semibold rounded-md hover:bg-blue-600"
            >
                "去往首页"
            </a>
        </div>
    }
}
