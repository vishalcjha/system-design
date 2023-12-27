use leptos::*;

use crate::topic::uber::entity::GridComponent;

#[component]
pub fn UberComponent() -> impl IntoView {
    view! {
        <div id="page-container">
        <div id="first" style="display: flex; flex-direction: column;justify-content: center; align-items: center; flex=1;">
            <img src="/public/uber_stream.png" alt="Compute Image"/>
            <div>
                <GridComponent/>
            </div>
        </div>
        <div id="second">
            <p> Describe uber system design </p>
        </div>
        </div>

    }
}
