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
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/system-design.css"/>

        <script>
        document.addEventListener("DOMContentLoaded", function () {
            const overlay = document.getElementById("overlay");
            const canvas = document.getElementById("canvas");

            function updateOverlaySize() {
            // Set canvas dimensions to the entire window
            canvas.width = window.innerWidth;
            canvas.height = window.innerHeight;
              const isLandscape = window.innerWidth > window.innerHeight;

              if (isLandscape) {
                // Set width to half of the window
                overlay.style.width = window.innerWidth / 2 + "px";
                overlay.style.height = "100%";
              } else {
                // Set height to half of the window
                overlay.style.width = "100%";
                overlay.style.height = window.innerWidth / 2 + "px";
              }
            }

            // Initial update
            updateOverlaySize();

            // Add event listener for window resize
            window.addEventListener("resize", updateOverlaySize);
          });
        </script>
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
        <main id="overlay" style="position: absolute;">
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
