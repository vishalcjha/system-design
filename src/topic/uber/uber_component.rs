use leptos::*;

use crate::topic::uber::entity::GridComponent;

#[component]
pub fn UberComponent() -> impl IntoView {
    view! {
        <div id="page-container">
        <div id="first">
            <div class="grid-container-16" id="uber-component">
            <img src="/public/uber_stream.png" alt="Compute Image" style="grid-column: 8; grid-row: 20"/>
            <div id="uber-grid-componene">
                <GridComponent/>
            </div>
            </div>
        </div>
        <div id="second">
            <p> Describe uber system design </p>
        </div>
        </div>

    }
}
