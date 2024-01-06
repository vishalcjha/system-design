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
    let (details_page, set_details_page) = create_signal("Intro");

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
                set_details_page.update(|current| *current = AllGoodScenario::NAME);
            });
        } else if selected_scenario == "client_deny" {
            set_sernario.update(|scenario| {
                *scenario = Rc::new(Some(Box::new(ClientDenyScenario::new())));
                set_details_page.update(|current| *current = ClientDenyScenario::NAME);
            });
        } else if selected_scenario == "server_down" {
            set_sernario.update(|scenario| {
                *scenario = Rc::new(Some(Box::new(ServerDownScenario::new())));
                set_details_page.update(|current| *current = ServerDownScenario::NAME);
            });
        } else if selected_scenario == "client_down" {
            set_sernario.update(|scenario| {
                *scenario = Rc::new(Some(Box::new(OneClientDown::new())));
                set_details_page.update(|current| *current = OneClientDown::NAME);
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
            flex:1;align-self:stretch;justify-self:stretch;
            padding:1px;border:solid green;">
            <div style="display:flex;flex-direction:row;flex:1;justify-content:space-between;align-items:center;">
            {move || computes().into_iter().map(|compute|
                view! {
                    <div>
                    <ComputeComponent compute={compute.1} id={compute.0} />
                    </div>
                }
            ).collect_view()}
            </div>

            <div style="display:flex;flex:1;justify-content:center;align-items:start;">
                <Show
                    when= move || has_scenario()
                    fallback=move || view! {
                        <select id="scenario_selector" class="button" on:change=selector_change_handler >
                        <option value="">Select Scenario</option>
                        <option value="all_good">All Good Scenario</option>
                        <option value="client_deny">Service Deny Scenario</option>
                        <option value="server_down">Orchestrator Down Scenario</option>
                        <option value="client_down">Service Down Scenario</option>
                    </select>
                    }
                >
                <button class="button"  on:click=button_handler>{button_label}</button>
                </Show>
            </div>
        </div>
        <div id="second" style="flex:1;;min-height:0;">
            {
                move || {
                    match details_page() {

                            AllGoodScenario::NAME => Some(view! {
                            <div>
                            <b>1.</b> Orchestrator asks for Prepare.<br/>
                            <b>2.</b> Both service return with Yes and individually reserve resouce for this transaction.<br/>
                            <b>3.</b> Orchestrator asks both to execute. And marks transaction successful.
                            </div>
                            }),
                            ClientDenyScenario::NAME => Some(view! {
                                <div>
                                <b>1.</b> Orchestrator asks for Prepare.<br/>
                                <b>2.</b> One or both service deny because it can not reserve resource. Transaction is aborted.<br/>
                                <b>3.</b> Orchestrator asks services to free up reserved resouce for other transaction.
                            </div>
                            }),
                            ServerDownScenario::NAME => Some(view! {
                                <div>
                                <b>1.</b> Orchestrator asks for Prepare.<br/>
                                <b>2.</b> Both service return with Yes with resouce in reserved state. But now server is down.<br/><br/>
                                Resources reserved during Prepare steps are locked and not available for other transaction.
                                This shows limitation of 2PC algorithm.
                                In general, implementation add notion of reservation timeout to overcome leaking resource locking.
                                </div>
                            }),
                            OneClientDown::NAME => Some(view! {
                                <div>
                                <b>1.</b> Orchestrator asks for Prepare.<br/>
                                <b>2.</b> Both service return with Yes with resouce in reserved state.<br/>
                                <b>3.</b> Server sends execute but Service2 is down.<br/>
                                <b>4.</b> Evetually Service2 comes up. It syncs with Orchestrator for transaction is has prepared,
                                but did not execute. It executes all such unexectued transactions.<br/> <br/>

                                </div>
                            }),
                        _ => Some(view! {
                            <div>
                            <h3>Why we need two phase commit</h3>
                            <p>With the widespread adoption of microservices, each owning its own database,
                            the complexity of managing transactions that span multiple services
                            has become non-trivial.
                            This complexity can result in challenges such as overbooking in certain
                            scenarios and underbooking in others,
                            both of which pose significant risks to business operations and customer satisfaction.
                            Unlike the pre-microservices era, where monolith applications could
                            seamlessly wrap multiple table mutations within a single transaction,
                            the decentralized nature of microservices demands a robust solution to ensure
                            transactional integrity.
                            </p>

                            <h3>Two phase commit: an attempt to achieve distributed transaction. </h3>
                            <p>Two-Phase Commit is a technique used to manage distributed transactions.
                            It employs an orchestrating service that coordinates the process.
                            The sequence involves two phases:</p>
                            <ol>
                            <li><b>Prepare Phase:</b> The orchestrator sends a request to all involved services,
                            asking if they can proceed with the transaction.
                            Each service responds based on its current state.</li>
                            <li><b>Execute Phase:</b> Upon receiving consent from all services in the prepare phase, the orchestrator executes the transaction.</li>
                            </ol>

                            <h3> Action time </h3>
                            <p> Use scenario button to see all possible flow of command and response.</p>
                            </div>
                        }),
                    }
                }
            }

        </div>
        </div>
    }
}
