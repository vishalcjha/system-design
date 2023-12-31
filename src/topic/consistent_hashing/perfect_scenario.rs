use std::collections::BTreeMap;

use leptos::*;

use crate::{
    model::node_hashing::{NodeHashing, Sliced},
    topic::consistent_hashing::initial_setup::InitialSetupComponent,
};

use super::Distribution;

fn get_initail(server_count: u32) -> (BTreeMap<u32, (Vec<u32>, Vec<u32>)>, Option<Distribution>) {
    if server_count == 3 {
        NodeHashing::consistent(Sliced::new(server_count, 19)).data_mapping(5, 16)
    } else {
        let sliced = Sliced::new(3, 19).updated_slice(server_count);
        NodeHashing::consistent(sliced).data_mapping(5, 16)
    }
}

#[component]
pub(super) fn PerfectScenario(
    server_count: ReadSignal<u32>,
    set_distribution: WriteSignal<Option<Distribution>>,
) -> impl IntoView {
    let setup = move || {
        let initials = get_initail(server_count());
        set_distribution.set(initials.1);
        initials.0
    };
    view! {
       <InitialSetupComponent server_count=server_count setup=Signal::derive(setup)/>
    }
}
