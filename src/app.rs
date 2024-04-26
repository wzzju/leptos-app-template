use charming::WasmRenderer;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/leptos-app-template.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

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
    let (show, set_show) = create_signal(false);
    let action = create_action(move |draw: &bool| {
        let draw = draw.clone();
        set_show(draw);
        async move {
            if draw {
                logging::log!("Get Chart");
                let chart = crate::line_chart::chart();
                logging::log!("Plot Chart");
                let renderer = WasmRenderer::new(800, 600);
                renderer.render("chart", &chart).unwrap();
            }
        }
    });

    view! {
        <div class="flex p-6 mx-auto justify-start space-x-6" id="main">
            <div id="left">
                <div class="flex space-y-6 flex-col">
                    <button
                        class="text-white bg-gradient-to-r from-cyan-400 via-cyan-500 to-cyan-600 hover:bg-gradient-to-br focus:ring-4 focus:outline-none focus:ring-cyan-300 dark:focus:ring-cyan-800 shadow-lg shadow-cyan-500/50 dark:shadow-lg dark:shadow-cyan-800/80 font-medium rounded-lg text-sm px-5 py-2.5 text-center me-2 mb-2"
                        on:click=move |_| {
                            action.dispatch(true);
                        }
                    >

                        "Show Chart"
                    </button>
                    <button
                        class="text-white bg-gradient-to-r from-teal-400 via-teal-500 to-teal-600 hover:bg-gradient-to-br focus:ring-4 focus:outline-none focus:ring-teal-300 dark:focus:ring-teal-800 shadow-lg shadow-teal-500/50 dark:shadow-lg dark:shadow-teal-800/80 font-medium rounded-lg text-sm px-5 py-2.5 text-center me-2 mb-2"
                        on:click=move |_| {
                            action.dispatch(false);
                        }
                    >

                        "Clear Chart"
                    </button>
                </div>
            </div>
            <div id="right">
                <Show
                    when=move || show()
                    fallback=|| view! { <b class="text-red-400">"Hello, charming!"</b> }
                >
                    <div id="chart"></div>
                </Show>
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

    view! { <h1>"Not Found"</h1> }
}
