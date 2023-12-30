use crate::{
    error_template::{AppError, ErrorTemplate},
    graphics::{clear_canvas, is_landscape},
    topic::{
        consistent_hashing::ConsistentHashingComponent, geo_hash::GeoHashComponent,
        two_phase::two_phase_commit::TwoPhaseCommit, uber::uber_component::UberComponent,
    },
};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();
    view! {

        // sets the document title
        <Title text="System Design"/>
        <Link rel="icon" href="/public/favicon.ico"/>
        // content for this welcome page
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! {
                <ErrorTemplate outside_errors/>
            }
            .into_view()
        }>
        <canvas id="canvas" style="z-index:-10"/>

        <Routes>
            <Route path="" view=HomePage/>
            <Route path="/2pc" view=TwoPhaseCommit/>
            <Route path="/geo-hash" view=GeoHashComponent/>
            <Route path="/uber" view=UberComponent/>
            <Route path="/consistent-hashing" view=ConsistentHashingComponent/>
            // <Route path="/server-client" view=/>
        </Routes>
        </Router>

    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    create_effect(|_| clear_canvas());
    let ref_button_map = vec![
        ("/geo-hash", "Geo Hash"),
        ("/2pc", "2 Phase Commit"),
        ("/consistent-hashing", "Consistenet Hashing"),
        // ("/caching", "Caching"),
        ("/uber", "Uber"),
    ];
    view! {
        <div style="display:flex;flex:1;flex-direction:row;justify-self:stretch;align-self:stretch;">
        <div id="first" style="display:flex;flex:1;flex-direction:column;
            align-self:stretch;
            border-color:green;
            justify-content:center;align-items:center;"
            style:border-style = move || if is_landscape() {"solid"} else {"none"}
        >

            {
                (!is_landscape()).then(move || view! {
                    <div id="second" style="align-self:center;order:20;position:fixed;bottom:0;margin-bottom: 5px;">
                        <h1> System Design </h1>
                        <a href="https://www.linkedin.com/in/vishal-kumar-46455425/"> Connect on LI  </a>
                        <br/>
                        System Desing on high level with interactive UI. I hope to make learning experience easy and enjoyable.
                    </div>
                })
            }
        <div class="topic">
        {move || ref_button_map.iter().map(|page_title| view! {
            <div style="display:flex;margin:10px">
            <a href={page_title.0}>{page_title.1}</a>
            </div>
        }).collect_view()}
        </div>

        </div>

        {
            is_landscape().then(move || view! {
                <div id="second" style="display:flex;flex:1;flex-direction:column;justify-content:center;align-items:center;">
                    <h1> System Design </h1>
                    System Desing on high level with interactive UI. I hope to make learning experience easy and enjoyable.
                    <br/>
                    <a href="https://www.linkedin.com/in/vishal-kumar-46455425/"> Connect on LI </a>
                </div>
            })
        }

        </div>
    }
}
