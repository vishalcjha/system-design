use leptos::*;

use crate::graphics::{clear_canvas, draw_grid_lines};

#[component]
pub fn UberComponent() -> impl IntoView {
    // Draw the grid line
    create_effect(move |_| {
        clear_canvas();
        draw_grid_lines(16);
    });

    view! {
        <div class="grid-container-16" id="uber-component">
        <img src="/uber_stream.png" alt="Compute Image" style="grid-column: 8; grid-row: 20"/>
        </div>
    }
}
