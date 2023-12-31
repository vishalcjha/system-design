use std::collections::BTreeMap;

use leptos::*;

use crate::{
    graphics::clear_canvas,
    model::node_hashing::{NodeHashing, NonSliced},
    topic::consistent_hashing::initial_setup::InitialSetupComponent,
};
fn get_initail(server_count: u32) -> BTreeMap<u32, (Vec<u32>, Vec<u32>)> {
    if server_count == 3 {
        NodeHashing::new_mod_based(NonSliced::new(server_count)).data_mapping(5, 16)
    } else {
        let non_sliced = NonSliced::new(3).updated(server_count);
        NodeHashing::new_mod_based(non_sliced).data_mapping(5, 16)
    }
}
#[component]
pub(super) fn ProblemScenario(server_count: ReadSignal<u32>) -> impl IntoView {
    create_effect(|_| clear_canvas());
    let setup = move || get_initail(server_count());

    view! {
       <InitialSetupComponent server_count=server_count setup=Signal::derive(setup)/>
    }
}
