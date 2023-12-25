use crate::{
    error_template::{AppError, ErrorTemplate},
    graphics::clear_canvas,
    topic::{two_phase::two_phase_commit::TwoPhaseCommit, uber::uber_component::UberComponent},
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
        <Title text="System Design with Fun"/>
        // content for this welcome page
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! {
                <ErrorTemplate outside_errors/>
            }
            .into_view()
        }>
        <canvas id="canvas"/>
        <main id="main">
            <Routes>
                <Route path="" view=HomePage/>
                <Route path="/2pc" view=TwoPhaseCommit/>
                <Route path="/uber" view=UberComponent/>
            </Routes>
        </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    create_effect(|_| clear_canvas());
    view! {
        <div class="card">
            <nav> <a href="/2pc"><button class="button" type="button"> 2 Phase Commit </button> </a></nav>
        </div>

        <div class="card">
            <nav> <a href="/uber"><button class="button" type="button"> Uber </button> </a></nav>
        </div>

        <div class="card">
            <nav> <a href="/caching"><button class="button" type="button"> Caching </button> </a></nav>
        </div>
    }
}
