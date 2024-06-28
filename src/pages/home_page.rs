use crate::components::PageTitle;
use leptos::*;
use leptos_bulma::elements::BButton;

#[component]
pub fn HomePage() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    view! {
        <PageTitle text="Home"/>
        <div class="has-text-centered">
            <h2 class="title is-3">"Welcome to Leptos!"</h2>
            <h3 class="subtitle">
                "This website is meant to be a guide on how to use it on your Leptos application."
            </h3>
            <BButton is_rounded=true on:click=on_click>
                "Click Me: "
                {count}
            </BButton>
        </div>
    }
}
