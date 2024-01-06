use leptos::*;

use crate::model::compute::{Compute, ComputeType, Status};

#[component]
pub fn ComputeComponent(compute: Compute, id: String) -> impl IntoView {
    let Compute {
        pos: _,
        status,
        compute_type,
    } = compute;
    view! {
     <div id={id} class="square-card"
        // style:grid-column = move || format!("{}", pos.1)
        // style:grid-row = move || format!("{}", pos.0)
    >
        <img class="img" src="/public/server.png" alt="Compute Image"/>
        <img class="icon" src={get_status_img(&status)} alt="Compute Status"/>
        <label class="label" style="">{get_label(&compute_type)}</label>
     </div>
    }
}

fn get_label(compute_type: &ComputeType) -> String {
    match compute_type {
        crate::model::compute::ComputeType::Server(ref name) => {
            String::from(name.clone().unwrap_or(String::from("Server")))
        }
        crate::model::compute::ComputeType::Client(num, ref name) => {
            format!("{}{}", name.clone().unwrap_or(String::from("Client")), num)
        }
    }
}

fn get_status_img(status: &Status) -> &'static str {
    match status {
        Status::Up => "/public/ok.png",
        Status::Down => "/public/down.png",
    }
}
