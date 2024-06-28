pub mod app;
pub mod components;
#[cfg(feature = "ssr")]
pub mod fileserv;
pub mod layout;
pub mod pages;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::app::*;
    console_error_panic_hook::set_once();
    leptos::mount_to_body(App);
}
