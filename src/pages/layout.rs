use crate::app::{AppLayoutContext, APP_BAR_HEIGHT};
use leptonic::{components::prelude::*, prelude::*};
use leptos::*;
use leptos_router::*;

#[component]
#[allow(clippy::too_many_lines)]
pub fn Layout(#[prop(optional)] children: Option<Children>) -> impl IntoView {
    let router_context = use_context::<RouterContext>();
    let is_doc: Memo<bool> = create_memo(move |_| {
        router_context
            .as_ref()
            .map(|router| router.pathname().get().starts_with("/doc"))
            .unwrap_or(false)
    });

    let app_layout_ctx = expect_context::<AppLayoutContext>();
    let AppLayoutContext {
        is_small,
        main_drawer_closed,
        set_main_drawer_closed,
        doc_drawer_closed,
        set_doc_drawer_closed,
        ..
    } = app_layout_ctx;

    // Make sure the doc_drawer is closed whenever we leave a documentation route.
    create_effect(move |_| {
        if !is_doc() {
            set_doc_drawer_closed(true);
        } else {
            if !is_small() {
                set_doc_drawer_closed(false);
            }
        }
    });

    // Always close the doc-drawer when the application is now small.
    // Always open the doc-drawer when the application is no longer small.
    create_effect(move |_| {
        if is_small() {
            set_doc_drawer_closed(true);
        } else {
            set_doc_drawer_closed(false);
        }
    });

    // Always close the main-drawer when the application is no longer small.
    create_effect(move |_| {
        if !is_small() {
            set_main_drawer_closed(true);
        }
    });

    let logo = move || {
        view! {
            <Link href="">
                <img src="/favicon.ico" id="logo" alt="Chat"/>
            </Link>
        }
    };

    view! {
        <AppBar id="app-bar" height=APP_BAR_HEIGHT>
            <div id="app-bar-content">
                <Stack id="left" orientation=StackOrientation::Horizontal spacing=Size::Zero>
                    {move || match (is_doc(), is_small()) {
                        (false, true) => logo().into_view(),
                        (true, true) => {
                            view! {
                                <Icon
                                    id="mobile-menu-trigger"
                                    icon=icondata::BsList
                                    on:click=move |_| app_layout_ctx.toggle_doc_drawer()
                                />
                                {logo}
                            }
                                .into_view()
                        }
                        (_, false) => {
                            view! {
                                {logo}
                                <Link href="/">
                                    <H3 style="margin: 0 0 0 0.5em">"Home"</H3>
                                </Link>
                            }
                                .into_view()
                        }
                    }}

                </Stack>

                <Stack id="right" orientation=StackOrientation::Horizontal spacing=Size::Em(1.0)>
                    {move || match is_small() {
                        true => {
                            view! {
                                <Icon
                                    id="mobile-menu-trigger"
                                    icon=icondata::BsThreeDots
                                    on:click=move |_| app_layout_ctx.toggle_main_drawer()
                                />
                            }
                                .into_view()
                        }
                        false => {
                            view! {
                                <Link href="/change">"v0.6.0 (main)"</Link>

                                <LinkExt
                                    href="https://github.com/lpotthast/leptonic"
                                    target=LinkExtTarget::Blank
                                >
                                    <Icon
                                        id="github-icon"
                                        icon=icondata::BsGithub
                                        aria_label="GitHub icon"
                                    />
                                </LinkExt>

                                <ThemeToggle
                                    off=LeptonicTheme::Light
                                    on=LeptonicTheme::Dark
                                    style="margin-right: 1em"
                                />
                            }
                                .into_view()
                        }
                    }}

                </Stack>
            </div>
        </AppBar>

        <Box
            id="content"
            attr:aria-hidden=move || {
                ((is_doc() && is_small() && !doc_drawer_closed()) || !main_drawer_closed())
                    .to_string()
            }
        >

            {match children {
                Some(children) => {
                    let children = children();
                    view! { {children} }.into_view()
                }
                None => {
                    view! {
                        // <Outlet/> will show nested child routes.
                        <Outlet/>
                    }
                        .into_view()
                }
            }}

            <Drawer
                id="main-drawer"
                shown=Signal::derive(move || !main_drawer_closed())
                side=DrawerSide::Right
                style=format!("top: {APP_BAR_HEIGHT}")
            >
                <Stack orientation=StackOrientation::Vertical spacing=Size::Em(2.0) class="menu">

                    <LinkExt
                        href="https://github.com/lpotthast/leptonic"
                        target=LinkExtTarget::Blank
                        style="font-size: 3em;"
                    >
                        <Icon id="github-icon" icon=icondata::BsGithub/>
                    </LinkExt>

                    <ThemeToggle
                        off=LeptonicTheme::Light
                        on=LeptonicTheme::Dark
                        style="margin-right: 1em"
                    />

                    "Currently - v0.6.0 (main)"
                </Stack>
            </Drawer>
        </Box>
    }
}
