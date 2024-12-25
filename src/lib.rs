#![allow(unused_parens)]
#![allow(non_snake_case)]

pub mod app;
mod front;
mod api;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::app::*;
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(App);
}