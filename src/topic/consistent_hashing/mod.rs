use leptos::*;

use crate::topic::consistent_hashing::{
    circle_distribution::CircleDistibutionComponent, perfect_scenario::PerfectScenario,
    problem_scenario::ProblemScenario, uneven_scenario::UnevenScenario,
};
mod circle_distribution;
mod initial_setup;
mod perfect_scenario;
mod problem_scenario;
mod uneven_scenario;

#[component]
pub(crate) fn ConsistentHashingComponent() -> impl IntoView {
    let (scenario, set_scenario) = create_signal(String::from("problem"));
    let (server_count, set_server_count) = create_signal(3);

    view! {
        <div id="page-container">

        <div id="first" style="dispaly:flex;flex:1;justify-self:stretch;align-self:stretch;
            padding:5px;border:solid green;flex-direction:column;justify-items:center;align-content:center;">
            <div id="scenario" style="flex:1;dispaly:flex;flex-direction:column;align-items:center;">
                // <div style="flex:1;margin-bottom:3px;"><InitialSetupComponent server_count=server_count/></div>
                <div style="flex:1">
                {
                    move || {
                        if scenario() == "perfect" {
                            view! {
                                <div>
                                <PerfectScenario server_count=server_count/>
                                </div>
                            }
                        } else if scenario() == "uneven" {
                            view! {<div><UnevenScenario server_count=server_count/></div>}
                        } else {
                            view! {<div><ProblemScenario server_count=server_count/></div>}
                        }
                    }
                }
                </div>

                <div id="circle-distribution-component" style="flex:1">
                {
                    move || {
                        if scenario() == "perfect" {
                            Some(view! {
                                <div>
                                <CircleDistibutionComponent element_id="circle-distribution-component" server_count=server_count/>
                                </div>
                            })
                        } else if scenario() == "uneven" {
                            Some(view! {<div><CircleDistibutionComponent element_id="circle-distribution-component" server_count=server_count/></div>})
                        } else {
                            None
                        }
                    }
                }
                </div>

                <div style="display:flex;flex-direction:row;justify-content:space-evenly;align-items:end;">
                {move || {
                    let mut views = vec![
                        view! {<div><button on:click=move |_| set_server_count(2) class="button">"2 Nodes"</button></div>},
                        view! {<div><button on:click=move |_| set_server_count(3) class="button">"3 Nodes"</button></div>},
                        view! {<div><button on:click=move |_| set_server_count(4) class="button">"4 Nodes"</button></div>}
                    ];
                    let removed_index = if server_count() == 2 { 0 } else if server_count() == 3 { 1 } else { 2 };
                    let _ = views.remove(removed_index);
                    let scenario_view = view! {
                        <div id="scenario-selector;">
                            <select on:change=move |cv| set_scenario(event_target_value(&cv))>
                                <option value="">Select Scenario</option>
                                <option value="problem">Mod Based</option>
                                <option value="uneven">Unbalanced Load</option>
                                <option value="perfect">Consitent Hashing</option>
                            </select>
                        </div>
                    };
                    views.insert(1, scenario_view);
                    views.collect_view()
                }}
                </div>

            </div>

        </div>

        <div id="second" style="dispaly:flex;flex:1;justify-self:stretch;align-self:stretch;">
                <p> Describe Things </p>
        </div>
        </div>
    }
}
