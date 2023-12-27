use std::rc::Rc;

use leptos::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlSelectElement;

use crate::{
    components::compute_component::ComputeComponent,
    graphics::clear_canvas,
    topic::two_phase::{
        all_good_scenario::AllGoodScenario, client_deny_scenario::ClientDenyScenario, get_computes,
        one_client_down::OneClientDown, server_down::ServerDownScenario, ComputeStatusChanger,
    },
};

#[component]
pub fn TwoPhaseCommit() -> impl IntoView {
    let (play, set_play) = create_signal(false);

    let (computes, set_computes) = create_signal(get_computes());

    let (scenario, set_sernario) =
        create_signal::<Rc<Option<Box<dyn ComputeStatusChanger>>>>(Rc::new(None));

    let has_scenario = move || {
        if let Some(_) = *scenario() {
            true
        } else {
            false
        }
    };

    create_effect(|_| clear_canvas());

    let button_handler = move |_| {
        set_sernario.update(|scenario| {
            let mut finished = false;
            if let Some(scenario) = scenario.as_ref() {
                if scenario.execute().is_none() {
                    finished = true;
                }

                if let Some(updated_computes) = scenario.status_updater(&computes()) {
                    logging::log!("Updted computes are {:?}", updated_computes);
                    set_computes.update(|computes| *computes = updated_computes);
                }
            }
            if finished {
                *scenario = Rc::new(None);
            }
        });
        set_play(true);
    };

    let selector_change_handler = move |_| {
        let selected_scenario = document()
            .get_element_by_id("scenario_selector")
            .unwrap()
            .dyn_into::<HtmlSelectElement>()
            .unwrap()
            .value();
        set_play.set(false);
        clear_canvas();
        set_computes.update(|computes| *computes = get_computes());
        if selected_scenario == "all_good" {
            set_sernario.update(|scenario| {
                *scenario = Rc::new(Some(Box::new(AllGoodScenario::new())));
            });
        } else if selected_scenario == "client_deny" {
            set_sernario.update(|scenario| {
                *scenario = Rc::new(Some(Box::new(ClientDenyScenario::new())));
            });
        } else if selected_scenario == "server_down" {
            set_sernario.update(|scenario| {
                *scenario = Rc::new(Some(Box::new(ServerDownScenario::new())));
            });
        } else if selected_scenario == "client_down" {
            set_sernario.update(|scenario| {
                *scenario = Rc::new(Some(Box::new(OneClientDown::new())));
            });
        }
    };

    let button_label = move || {
        if play() {
            "Next"
        } else {
            "Start"
        }
    };

    view! {
        <div id="page-container">
        <div id="first" style="display:flex;flex-direction:column;
            flex:1;align-self:stretch;align-items:stretch;justify-content:stretch;justify-self:stretch;
            padding:5px;border-style:solid;border-color:green;">
            <div style="display:flex;flex-direction:row;flex:1;justify-content:stretch;">
            {move || computes().into_iter().map(|compute|
                view! {
                    <div style="display:flex;flex:1">
                    <ComputeComponent compute={compute.1} id={compute.0} />
                    </div>
                }
            ).collect_view()}
            </div>

            <div style="display:flex;flex:1;justify-content:center;align-items:center;align-self:center;justify-self:center;">
                <Show
                    when= move || has_scenario()
                    fallback=move || view! {
                        <select id="scenario_selector" class="button" on:change=selector_change_handler >
                        <option value="">Select Scenario</option>
                        <option value="all_good">All Good Scenario</option>
                        <option value="client_deny">Client Deny Scenario</option>
                        <option value="server_down">Server Down Scenario</option>
                        <option value="client_down">Client Down Scenario</option>
                    </select>
                    }
                >
                <button class="button"  on:click=button_handler>{button_label}</button>
                </Show>
            </div>
        </div>
        <div id="second" style="flex:1;">
            <p> describe what is going on </p>
        </div>
        </div>
    }
}
