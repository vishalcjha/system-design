use std::collections::BTreeMap;

use leptos::*;

#[component]
pub(super) fn InitialSetupComponent() -> impl IntoView {
    let initial_setup = (1..20).fold(BTreeMap::<u32, Vec<u32>>::new(), |mut accum, current| {
        accum
            .entry(current % 3)
            .and_modify(|nums| nums.push(current))
            .or_insert_with(|| vec![current]);
        accum
    });

    view! {
        <div style="border-style:solid;border-color:yellow;padding:1em;padding: 1em;">
            <div style="justify-content:center;">
                <b> Initial Setup </b>
            </div>
            {initial_setup.iter().map(|(key, value)| view! {
                <p> Server {key + 1} -> {format!("{:?}", value)} </p>
            }).collect_view()}
        </div>
    }
}
