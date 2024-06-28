use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::layout::Layout;
use crate::pages::*;
use leptos_use::{use_color_mode_with_options, UseColorModeOptions, UseColorModeReturn};

pub fn use_app_color_mode() -> UseColorModeReturn {
    use_color_mode_with_options(
        UseColorModeOptions::default()
            .cookie_enabled(true)
            .cookie_name("pref_color_mode")
            .attribute("data-theme")
            .emit_auto(true)
            .transition_enabled(true),
    )
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/{{project-name}}.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        <div class="loading-overlay" class:is-done=leptos_dom::is_browser></div>

        // content for this welcome page
        <Router
            trailing_slash=TrailingSlash::Redirect
            fallback=|| { view! { <NotFoundPage/> }.into_view() }
        >
            <Layout>
                <Routes>
                    <Route path="/" view=HomePage/>
                    <Route path="/form" view=FormPage/>
                </Routes>
            </Layout>
        </Router>
    }
}
