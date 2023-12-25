use crate::app::App;

pub mod app;
pub mod components;
pub mod dark_mode;
pub mod error_template;
pub mod graphics;
mod topic;

pub mod model;
pub fn main() {
    use leptos::*;
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    logging::log!("csr mode - mounting to body");

    mount_to_body(|| {
        view! { <App/> }
    });
}
