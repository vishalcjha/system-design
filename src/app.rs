use crate::{
    dark_mode::DarkmodeToggle,
    error_template::{AppError, ErrorTemplate},
    two_phase_commit::TwoPhaseCommit,
};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {


        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/system-design.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! {
                <ErrorTemplate outside_errors/>
            }
            .into_view()
        }>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                    <Route path="/2pc" view=TwoPhaseCommit/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    view! {
        <div class="card">
            <nav> <a href="/2pc"><button class="button" type="button"> 2 Phase Commit </button> </a></nav>
        </div>

        <div class="card">
        <nav> <a href="/caching"><button class="button" type="button"> Caching </button> </a></nav>
    </div>
    }
}
