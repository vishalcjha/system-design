use leptos::*;

use crate::{
    components::compute_component::ComputeComponent,
    model::compute::{Compute, Pos},
    two_phase::{all_good_scenario::AllGoodScenario, PositionHolder},
};

#[component]
pub fn TwoPhaseCommit() -> impl IntoView {
    let (play, set_play) = create_signal(false);
    let button_handler = move |_| {
        if !play() {
            let all_good_scenario = AllGoodScenario::default();
            all_good_scenario.execute();
        }
        set_play.update(move |current| *current = !current.clone());
    };
    let button_label = move || {
        if play() {
            "Stop"
        } else {
            "Start"
        }
    };
    let computes = vec![
        (
            String::from(PositionHolder::SERVER),
            Compute::new_server(Pos(4, 4)),
        ),
        (
            String::from(PositionHolder::CLIENT_ONE),
            Compute::new_client(Pos(4, 1)),
        ),
        (
            String::from(PositionHolder::CLIENT_TWO),
            Compute::new_client(Pos(4, 8)),
        ),
    ];

    view! {
        <div class="grid-container" id="two_phase_commit">
        {computes.into_iter().map(|compute|
            view! {
                <ComputeComponent compute={compute.1} id={compute.0} />
            }
        ).collect_view()}
        <button class="button" style="grid-row:10; grid-column:4" on:click=button_handler>{button_label}</button>
        </div>

    }
}
