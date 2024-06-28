use crate::components::PageTitle;
use leptos::*;
use leptos_bulma::{
    columns::{BColumn, BColumns},
    elements::{BBox, BButton, BSubtitle, BTitle},
    enums::BColor,
    layout::BSection,
};

#[component]
pub fn HomePage() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    view! {
        <PageTitle text="Home"/>

        <BSection class="has-text-centered">
            <BTitle>"Welcome to Leptos!"</BTitle>
            <BSubtitle>
                "This website is meant to be a guide on how to use it on your Leptos application."
            </BSubtitle>
            <BButton is_rounded=true color=BColor::Primary on:click=on_click>
                "Click Me: "
                {count}
            </BButton>
        </BSection>

        <BSection class="has-text-centered">
            <BColumns>
                <BColumn is="two-fifths">
                    <BBox class="has-background-primary has-text-black">"First column"</BBox>
                </BColumn>
                <BColumn is="3">
                    <BBox class="has-background-light has-text-black">"Second column"</BBox>
                </BColumn>
                <BColumn>
                    <BBox class="has-background-info has-text-black">"Third column"</BBox>
                </BColumn>
                <BColumn is="narrow">
                    <BBox class="has-background-success has-text-black">"Fourth column"</BBox>
                </BColumn>
            </BColumns>
        </BSection>
    }
}
