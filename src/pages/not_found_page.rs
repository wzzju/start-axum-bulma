use leptos::*;

use crate::components::PageTitle;

/// 404 - Not Found
#[component]
pub fn NotFoundPage() -> impl IntoView {
    view! {
        <PageTitle text="Error 404: Page Not Found"/>

        <div class="has-text-centered">
            <h2 class="title is-3">"Uh oh!"</h2>
            <h3 class="subtitle">"We couldn't find that page!"</h3>
        </div>
    }
}
