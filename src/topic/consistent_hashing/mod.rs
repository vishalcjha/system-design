use leptos::*;

use crate::topic::consistent_hashing::{
    perfec_scenario::PerfectScenario, problem_scenario::ProblemScenario,
    uneven_scenario::UnevenScenario,
};
mod initial_setup;
mod perfec_scenario;
mod problem_scenario;
mod uneven_scenario;

#[component]
pub(crate) fn ConsistentHashingComponent() -> impl IntoView {
    let (scenario, set_scenario) = create_signal("problem");
    let (server_count, set_server_count) = create_signal(3);
    let server_count_inc = move |_| {
        set_server_count.update(|count| {
            if *count <= 5 {
                *count = *count + 1
            }
        });
    };
    let server_count_dec = move |_| {
        set_server_count.update(|count| {
            if *count >= 2 {
                *count = *count - 1
            }
        });
    };
    let button_handler = |id: &'static str| {
        move |_| {
            set_scenario(id);
        }
    };
    view! {
        <div id="page-container">

        <div id="first" style="dispaly:flex;flex:1;justify-self:stretch;align-self:stretch;
            padding:5px;border-style:solid;border-color:green;flex-direction:column;justify-items:center;align-content:center">
            <div id="scenario" style="flex:1;dispaly:flex;flex-direction:column;">

                <div>
                <Show when=move || scenario() == "problem"
                fallback=move|| view!{
                    <Show
                    when= move || scenario() == "perfect"
                    fallback=move || view! {<UnevenScenario server_count=server_count/>}>
                        <PerfectScenario server_count=server_count/>
                    </Show>
                }
                >
                    <ProblemScenario server_count=server_count/>
                </Show>
                </div>

                <div style="display:flex;flex-direction:row;justify-content:center;align-items:center;">
                <button on:click=server_count_dec class="button" style="color:green;">"-"</button>
                <p>    Server   </p>
                <button on:click=server_count_inc class="button" style="color:green;">"+"</button>
                </div>

            </div>
            <div id="scenario-selector" style="flex:1;dispaly:flex;
                flex-direction:row;justify-content: space-between;">
                <button class="button" id="problem" style="flex:1" on:click=button_handler("problem")>Problem</button>
                <button class="button" id="uneven" style="flex:1" on:click=button_handler("uneven")>Uneven</button>
                <button class="button"  id="perfect" style="flex:1" on:click=button_handler("perfect")>Perfect</button>
            </div>
        </div>

        <div id="second" style="dispaly:flex;flex:1;justify-self:stretch;align-self:stretch;">

        </div>
        </div>
    }
}
