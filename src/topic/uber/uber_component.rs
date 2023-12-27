use leptos::*;

use crate::topic::uber::entity::GridComponent;

#[component]
pub fn UberComponent() -> impl IntoView {
    view! {
        <div id="page-container">
        <div id="first" style="flex=1;align-self:stretch">
            <img src="/public/uber_stream.png" alt="Compute Image"/>
            <div>
                <GridComponent/>
            </div>
        </div>
        <div id="second" style="display:flex;flex=1;align-self:stretch;">
            <p> Describe uber system design </p>
        </div>
        </div>

    }
}
