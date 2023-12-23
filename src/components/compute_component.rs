use leptos::*;

use crate::model::compute::Compute;

#[component]
pub fn ComputeComponent(compute: Compute, id: String) -> impl IntoView {
    let Compute {
        pos,
        status,
        compute_type,
    } = compute;
    view! {
     <div id={id} class="square-card"
        style:grid-column = move || format!("{}", pos.1)
        style:grid-row = move || format!("{}", pos.0)
    >
        <img src="/server.png" alt="Compute Image"/>
        <div class="icon">{"✨"}</div>
     </div>
    }
}
