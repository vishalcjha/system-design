use leptos::*;
use leptos_meta::Meta;

#[component]
pub fn DarkmodeToggle() -> impl IntoView {
    let (prefers_dark, set_prefers_dark) = create_signal(false);
    let toggle_dart_mode = move |_| set_prefers_dark.update(|current| *current = !*current);
    let mode_name = move || if prefers_dark() { "Light " } else { "Dark " };

    let color_scheme = move || {
        if prefers_dark() {
            "dark"
        } else {
            "light"
        }
    };

    view! {
        <Meta name="color-scheme" content=color_scheme/>
        <button class="button" type="button" on:click=toggle_dart_mode> <span>{mode_name}</span> Mode</button>
    }
}
