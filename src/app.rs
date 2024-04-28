use charming::WasmRenderer;
use csv::Reader;
use futures_util::StreamExt;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use std::{io::Cursor, vec};
use wasm_streams::ReadableStream;
use web_sys::{js_sys::Uint8Array, wasm_bindgen::JsCast, File, FileList};

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/leptos-app-template.css"/>

        // sets the document title
        <Title text="Loss Comparison"/>

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
    let (lhs_ready, set_lhs_ready) = create_signal(false);
    let (rhs_ready, set_rhs_ready) = create_signal(false);
    let draw_ready =
        move || with!(|lhs_ready, rhs_ready| { lhs_ready.clone() && rhs_ready.clone() });

    let (lhs, set_lhs) = create_signal(String::new());
    let (rhs, set_rhs) = create_signal(String::new());

    let read_file = create_action(
        move |data: &(File, WriteSignal<String>, WriteSignal<bool>)| {
            let file = data.0.clone();
            let set_content = data.1.clone();
            let set_ready = data.2.clone();
            logging::log!("File name: {}", file.name());
            logging::log!("File last modified time: {}", file.last_modified());
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
                        set_ready(true);
                    }
                }
            }
        },
    );

    let draw_action = create_action(move |draw: &bool| {
        let draw = draw.clone();
        set_show(draw);
        async move {
            if draw {
                let mut x_data = vec![];
                let mut lhs_data = vec![];
                let mut rhs_data = vec![];
                let mut lhs_rdr = Reader::from_reader(Cursor::new(lhs()));
                let mut rhs_rdr = Reader::from_reader(Cursor::new(rhs()));
                for result in lhs_rdr.records() {
                    let record = result.expect("a CSV record");
                    let x = record[0].to_string();
                    let y: f32 = record[1].parse().expect("Not a valid f64 number");
                    x_data.push(x);
                    lhs_data.push(y);
                }
                for result in rhs_rdr.records() {
                    let record = result.expect("a CSV record");
                    let y: f32 = record[1].parse().expect("Not a valid f64 number");
                    rhs_data.push(y);
                }
                let chart =
                    crate::line_chart::chart(&["XPU", "GPU"], x_data, &[lhs_data, rhs_data]);
                let renderer = WasmRenderer::new(800, 600);
                renderer.render("chart", &chart).unwrap();
            }
        }
    });

    view! {
        <div class="flex p-6 mx-auto justify-start space-x-6" id="main">
            <div id="left">
                <div class="flex space-y-6 flex-col">
                    <input
                        class="w-full text-gray-400 font-semibold text-sm bg-white border file:cursor-pointer cursor-pointer file:border-0 file:py-3 file:px-4 file:mr-4 file:bg-gray-100 file:hover:bg-gray-200 file:text-gray-500 rounded"
                        type="file"
                        name="file_upload"
                        multiple
                        on:input=move |ev| {
                            set_lhs_ready(false);
                            set_rhs_ready(false);
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
                                    match file.name().find("xpu") {
                                        Some(_index) => {
                                            read_file.dispatch((file, set_lhs, set_lhs_ready))
                                        }
                                        None => {
                                            if let Some(_index) = file.name().find("gpu") {
                                                read_file.dispatch((file, set_rhs, set_rhs_ready));
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    />

                    <p class="text-xs text-gray-400 mt-2">Only CSV is Allowed.</p>

                    <button
                        class="text-white bg-gradient-to-r from-cyan-400 via-cyan-500 to-cyan-600 hover:bg-gradient-to-br focus:ring-4 focus:outline-none focus:ring-cyan-300 dark:focus:ring-cyan-800 shadow-lg shadow-cyan-500/50 dark:shadow-lg dark:shadow-cyan-800/80 font-medium rounded-lg text-sm px-5 py-2.5 text-center me-2 mb-2"
                        disabled=move || !draw_ready()
                        on:click=move |_| {
                            draw_action.dispatch(true);
                        }
                    >

                        "Show Chart"
                    </button>
                    <button
                        class="text-white bg-gradient-to-r from-teal-400 via-teal-500 to-teal-600 hover:bg-gradient-to-br focus:ring-4 focus:outline-none focus:ring-teal-300 dark:focus:ring-teal-800 shadow-lg shadow-teal-500/50 dark:shadow-lg dark:shadow-teal-800/80 font-medium rounded-lg text-sm px-5 py-2.5 text-center me-2 mb-2"
                        on:click=move |_| {
                            draw_action.dispatch(false);
                        }
                    >

                        "Clear Chart"
                    </button>

                </div>
            </div>
            <div id="right">
                <Show when=move || show() fallback=|| view! {}>
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
