use leptos::*;

#[component]
pub(super) fn UnevenScenario(server_count: ReadSignal<u32>) -> impl IntoView {
    view! {
        <p> Uneven Scenario {move ||server_count()} </p>
    }
}
