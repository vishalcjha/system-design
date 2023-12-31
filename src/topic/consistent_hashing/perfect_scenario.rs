use std::collections::BTreeMap;

use leptos::*;

use crate::{
    model::node_hashing::{NodeHashing, Sliced},
    topic::consistent_hashing::initial_setup::InitialSetupComponent,
};

fn get_initail(server_count: u32) -> BTreeMap<u32, (Vec<u32>, Vec<u32>)> {
    if server_count == 3 {
        NodeHashing::consistent(Sliced::new(server_count, 19)).data_mapping(5, 16)
    } else {
        let sliced = Sliced::new(3, 19).updated_slice(server_count);
        NodeHashing::consistent(sliced).data_mapping(5, 16)
    }
}

#[component]
pub(super) fn PerfectScenario(server_count: ReadSignal<u32>) -> impl IntoView {
    let setup = move || get_initail(server_count());
    view! {
       <InitialSetupComponent server_count=server_count setup=Signal::derive(setup)/>
    }
}
