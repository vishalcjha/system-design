use leptos::*;

use crate::model::compute::{Compute, ComputeType, Status};

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
        <img class="img" src="/server.png" alt="Compute Image"/>
        <img class="icon" src={get_status_img(&status)} alt="Compute Statue"/>
        <label class="label">{get_label(&compute_type)}</label>
     </div>
    }
}

fn get_label(compute_type: &ComputeType) -> String {
    match compute_type {
        crate::model::compute::ComputeType::Server => String::from("Server"),
        crate::model::compute::ComputeType::Client(num) => format!("Client{}", num),
    }
}

fn get_status_img(status: &Status) -> &'static str {
    match status {
        Status::Up => "/ok.png",
        Status::Down => "/down.png",
    }
}
