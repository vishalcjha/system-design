use leptos::*;

use crate::topic::uber::entity::GridComponent;

#[component]
pub fn UberComponent() -> impl IntoView {
    view! {
        <div id="page-container">
        <div id="first" style="display:flex;flex-direction:column;
        flex:1;align-self:stretch;justify-self:stretch;
        justify-content:center;align-items:center;
        border:solid green;">
            <img src="/public/uber_stream.png" alt="Compute Image"/>
            <div>
                <GridComponent/>
            </div>
        </div>
        <div id="second" style="display:flex;flex-direction:column;
        justify-content:center;align-items:center;
        flex:1;align-self:stretch;justify-self:stretch;">
            <p> Describe uber system design </p>
        </div>
        </div>

    }
}
