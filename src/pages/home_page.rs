use crate::app::use_app_color_mode;
use crate::components::PageTitle;
use leptos::*;
use leptos_bulma::{
    columns::{BColumn, BColumns},
    elements::{BBox, BButton, BSubtitle, BTitle},
    enums::BColor,
    layout::BSection,
};
use leptos_toaster::{Theme, Toast, ToastId, ToastOptions, ToastVariant, ToasterPosition, Toasts};
use leptos_use::ColorMode;

#[component]
pub fn HomePage() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    let toast_theme = move || {
        if use_app_color_mode().mode.get() == ColorMode::Dark {
            Theme::Dark
        } else {
            Theme::Light
        }
    };
    let toast_context = expect_context::<Toasts>();
    let create_toast = move |_| {
        let toast_id = ToastId::new();
        toast_context.toast(
            view! {
                <Toast
                    toast_id
                    variant=ToastVariant::Info
                    theme=toast_theme()
                    invert=false
                    rich_colors=true
                    title=view! { "Title" }.into_view()
                    description=Some(view! { "Description" }.into_view())
                />
            },
            Some(toast_id),
            Some(ToastOptions {
                dismissible: true,
                duration: Some(std::time::Duration::from_secs(10)),
                position: Some(ToasterPosition::BottomRight),
            }),
        );
    };

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
            <BButton is_outlined=true color=BColor::Success on:click=create_toast>
                "Show Toast"
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
