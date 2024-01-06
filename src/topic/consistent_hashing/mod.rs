use std::collections::HashMap;

use leptos::*;

use crate::{
    graphics::{clear_canvas, is_landscape},
    topic::consistent_hashing::{
        circle_distribution::CircleDistibutionComponent, perfect_scenario::PerfectScenario,
        problem_scenario::ProblemScenario, uneven_scenario::UnevenScenario,
    },
};
mod circle_distribution;
mod initial_setup;
mod perfect_scenario;
mod problem_scenario;
mod uneven_scenario;

pub(self) type Distribution = (HashMap<u32, u32>, HashMap<u32, Vec<u32>>);

#[component]
pub(crate) fn ConsistentHashingComponent() -> impl IntoView {
    let (scenario, set_scenario) = create_signal(String::from("problem"));
    let (server_count, set_server_count) = create_signal(3);
    let (distribution, set_distribution) = create_signal::<Option<Distribution>>(None);

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
                                <PerfectScenario server_count=server_count set_distribution=set_distribution/>
                                </div>
                            }
                        } else if scenario() == "uneven" {
                            view! {<div><UnevenScenario server_count=server_count set_distribution=set_distribution/></div>}
                        } else if scenario() == "mod" {
                            view! {<div><ProblemScenario server_count=server_count/></div>}
                        } else {
                            view! {<div>
                            <h3> Resource Details </h3>
                            We have 2 - 4 servers. These are representer by number 1 to 4.
                            <h3> Workload Details </h3>
                            We have data or workload that when hashed get value from 5 to 12.
                            Note for similcity we have used resouce and workload numbers which are disjoint.
                            These numbers need not to be sequential.

                            </div>}
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
                                <CircleDistibutionComponent element_id="circle-distribution-component"  distribution=distribution/>
                                </div>
                            })
                        } else if scenario() == "uneven" {
                            Some(view! {<div><CircleDistibutionComponent element_id="circle-distribution-component"  distribution=distribution/></div>})
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
                            <select on:change=move |cv| {
                                clear_canvas();
                                set_scenario(event_target_value(&cv));
                            }>
                                <option value="problem">Select Scenario</option>
                                <option value="mod">Mod Based</option>
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
        {
            move || {
                match scenario().as_str() {
                    "perfect" => view! {
                        <div>
                        <h3>Placing Resource (duplicates) and Workload on a Circle</h3>
                        <p>We can imporve upon pervios distribution by assigning resouce multiple points on circle.
                        Workload is assigned single point. This satisfies both constraint. This is referred as <b>Consistent Hashing.</b></p>

                        <h5> See picture on <Show  when=move || is_landscape() fallback= move || view! { top }> left </Show> </h5>
                        Number 1-3, representing servers appers twice.
                        <p>This also opens room for taking account of resource capacity. Rather than same count of points on circle,
                        for each resource, one can use variable count depending on resouce capacity.
                        </p>
                        </div>
                    },
                    "uneven" => view! {
                        <div>
                        <h3>Placing Resource and Workload on a Circle</h3>
                        <p>In this step, each node and resource is assigned a point on a circle.
                        It is important to note that multiple nodes and resources can be assigned the same point.
                        The assignment follows a simple rule: Start from the resource point and move in an
                        anti-clockwise direction, and assign resource to first node encountered.</p>
                        <p>While this method achieves fair stickiness, it falls short in meeting the first constraint
                        of balanced distribution.</p>
                        </div>
                    },
                    "mod" => view! {
                        <div>
                        <h3>Mod Based Distribution</h3>
                        <p>In Mod Based Distribution, workload is assigned by taking the modulus with the resource count,
                        achieving a balanced distribution initially.</p>
                        <p>However, this method fails to maintain stickiness when the resource count changes.
                        Selecting a different node count results in a substantial shuffle,
                        failing to meet the second requirement of preserving workload assignments.</p>
                        <p>Explore the details of Mod Based Distribution by trying different node counts
                        and observe the significant shuffle that occurs.</p>
                        </div>
                    },
                    _ => view! {
                        <div>
                        <h3>Problem Statement</h3>
                        <p>In the realm of scalable and distributed systems, the consistent hashing problem arises
                        when faced with the challenge of distributing workload among a dynamic set of resources.</p>
                        <p>Given a set of resources, which can dynamically grow or shrink,
                        the goal is to efficiently distribute the workload among them.
                        Resources could be sharded database nodes, and the workload may represent,
                        for example, the user details table.</p>
                        <h4>Two key constraints guide our solution:</h4>

                        <b>1. Balance:</b> Achieving a balanced distribution of workload among resources for
                        efficient resource utilization.<br/>
                        <b>2. Stickiness:</b> Maintaining the stickiness of workload with resources.
                        Once a workload is assigned to a specific resource, we aim to preserve this assignment
                        even during resource growth and shrinkage, eliminating the need for reshuffling.

                        <h4>Select different secnario to see distribution.</h4>
                        </div>

                    }

                }
            }
        }
        </div>
        </div>
    }
}
