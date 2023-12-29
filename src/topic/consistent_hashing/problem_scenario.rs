use std::collections::BTreeMap;

use leptos::*;

use crate::topic::consistent_hashing::initial_setup::InitialSetupComponent;

#[component]
pub(super) fn ProblemScenario(server_count: ReadSignal<u32>) -> impl IntoView {
    let current = move || {
        (1..20).fold(BTreeMap::<u32, Vec<u32>>::new(), |mut accum, current| {
            accum
                .entry(current % server_count())
                .and_modify(|nums| nums.push(current))
                .or_insert_with(|| vec![current]);
            accum
        })
    };

    view! {
        <div>
        <InitialSetupComponent/>
        <div style="border-style:solid;border-color:blue;">
            <For
                each=current
                key=|x| format!("{:?}", x.1)
                children=move |(id, what) | {
                    view! {
                        <p> Server {id + 1 } -> {format!("{:?}", what)}  </p>
                    }
                }
            />
        </div>

        </div>
    }
}
