use charming::WasmRenderer;
use futures_util::StreamExt;
use leptos::{html::Textarea, *};
use leptos_meta::*;
use leptos_router::*;
use leptos_use::use_resize_observer;
use wasm_streams::ReadableStream;
use web_sys::{js_sys::Uint8Array, wasm_bindgen::JsCast, File, FileList};

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

    let (content, set_content) = create_signal("no content".to_string());

    let read_file = create_action(move |file: &File| {
        logging::log!("File name: {}", file.name());
        logging::log!("File last modified time: {}", file.last_modified());
        let file = file.clone();
        async move {
            let js_stream = ReadableStream::from_raw(file.stream());
            let mut stream = js_stream.into_stream();

            while let Some(Ok(chunk)) = stream.next().await {
                // web_sys::console::log_1(&chunk);
                let content_u8: Uint8Array = chunk.into();
                let content: Vec<u8> = content_u8.to_vec();
                if let Ok(content) = String::from_utf8(content) {
                    // logging::log!("{}", content);
                    set_content(content);
                }
            }
        }
    });
    let el = create_node_ref::<Textarea>();
    use_resize_observer(el, move |_, _| {});

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
                    <input
                        class="w-full text-gray-400 font-semibold text-sm bg-white border file:cursor-pointer cursor-pointer file:border-0 file:py-3 file:px-4 file:mr-4 file:bg-gray-100 file:hover:bg-gray-200 file:text-gray-500 rounded"
                        type="file"
                        name="file_upload"
                        multiple
                        on:input=move |ev| {
                            let files: FileList = ev
                                .target()
                                .unwrap()
                                .unchecked_ref::<web_sys::HtmlInputElement>()
                                .files()
                                .unwrap();
                            let len = files.length();
                            logging::log!("Files length: {}", len);
                            for i in 0..len {
                                if let Some(file) = files.get(i) {
                                    read_file.dispatch(file);
                                }
                            }
                        }
                    />

                    <p class="text-xs text-gray-400 mt-2">TXT, CSV and Excel are Allowed.</p>

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
        <textarea
            node_ref=el
            readonly
            class="resize rounded-md p-8 w-[200px] h-[100px]"
            prop:value=move || content.get()
        ></textarea>
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
