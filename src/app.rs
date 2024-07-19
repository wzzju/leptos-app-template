use crate::pages::*;
use leptonic::{components::prelude::*, prelude::*};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use leptos_use::use_media_query;

pub const APP_BAR_HEIGHT: Height = Height::Em(3.5);

#[derive(Debug, Clone, Copy)]
pub struct AppLayoutContext {
    pub is_small: Signal<bool>,
    pub is_medium: Signal<bool>,
    pub main_drawer_closed: ReadSignal<bool>,
    pub set_main_drawer_closed: WriteSignal<bool>,
    pub doc_drawer_closed: ReadSignal<bool>,
    pub set_doc_drawer_closed: WriteSignal<bool>,
}

impl AppLayoutContext {
    #[allow(unused)]
    pub fn close_main_drawer(&self) {
        (self.set_main_drawer_closed)(true);
    }

    pub fn close_doc_drawer(&self) {
        (self.set_doc_drawer_closed)(true);
    }

    pub fn toggle_main_drawer(&self) {
        let currently_closed = self.main_drawer_closed.get_untracked();
        (self.set_main_drawer_closed)(!currently_closed);
        if currently_closed {
            self.close_doc_drawer();
        }
    }

    pub fn toggle_doc_drawer(&self) {
        let currently_closed = self.doc_drawer_closed.get_untracked();
        (self.set_doc_drawer_closed)(!currently_closed);
        if currently_closed {
            self.close_main_drawer();
        }
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    let is_small = use_media_query("(max-width: 800px)");
    let is_medium = use_media_query("(max-width: 1200px)");
    // The main drawer is only used on mobile / small screens!.
    let (main_drawer_closed, set_main_drawer_closed) = create_signal(true);
    let (doc_drawer_closed, set_doc_drawer_closed) = create_signal(false);

    let app_layout_ctx = AppLayoutContext {
        is_small,
        is_medium,
        main_drawer_closed: main_drawer_closed,
        set_main_drawer_closed,
        doc_drawer_closed: doc_drawer_closed,
        set_doc_drawer_closed,
    };
    provide_context(app_layout_ctx);

    view! {
        <Stylesheet id="leptos" href="/pkg/ai-chat.css"/>

        // sets the document title
        <Title text="Welcome to AiChat"/>

        // content for this welcome page
        <Root default_theme=LeptonicTheme::default()>
            <Router trailing_slash=TrailingSlash::Redirect>
                <Layout>
                    <Routes>
                        <Route path="" view=HomePage/>
                        <Route path="/*any" view=NotFound/>
                    </Routes>
                </Layout>
            </Router>
        </Root>
    }
}
