use leptos::*;

#[component]
pub(super) fn PerfectScenario(server_count: ReadSignal<u32>) -> impl IntoView {
    view! {
        <p> Perfect Scenario {move ||server_count()} </p>
    }
}
