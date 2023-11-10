use cfg_if::cfg_if;
pub mod api;
pub mod client_app_state;
pub mod components;
pub mod error_template;
pub mod errors;
pub mod forms;
pub mod landing;
cfg_if! {
if #[cfg(feature = "hydrate")] {
// Needs to be in lib.rs AFAIK because wasm-bindgen needs us to be compiling a lib. I may be wrong.
use wasm_bindgen::prelude::wasm_bindgen;
use leptos::view;
use crate::landing::Landing;
}}
cfg_if! {
if #[cfg(feature = "ssr")] {
pub mod fallback;
pub mod server;
pub mod server_app_state;
}}

cfg_if! {
if #[cfg(feature = "hydrate")] {
        #[wasm_bindgen]
    pub fn hydrate() {
        _ = console_log::init_with_level(log::Level::Debug);
        console_error_panic_hook::set_once();

        leptos::mount_to_body(|| {
            view! { <Landing/> }
        });
    }
}}
