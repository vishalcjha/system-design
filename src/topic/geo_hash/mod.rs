use leptos::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlSelectElement;

use crate::{
    graphics::{find_element_pos, geo_hash::draw_geo_hash, get_window_x_y, is_landscape},
    model::geo_hash::Precision,
};

#[component]
pub fn GeoHashComponent() -> impl IntoView {
    let (clicked, set_clicked) = create_signal(false);
    let (precision, set_precision) = create_signal(Precision::ONE);
    let (point_clicked, set_point_clicked) = create_signal::<Option<(f64, f64)>>(None);
    let clicked_pos = move || {
        if let Some(point_clicked) = point_clicked() {
            let (lon, lat, geo_hash) = draw_geo_hash(point_clicked, precision());
            return format!(
                "[{}\u{00B0} {}\u{00B0}] {}",
                lon.floor(),
                lat.floor(),
                *geo_hash
            );
        }
        String::from("")
    };

    let handler = window_event_listener(ev::pointerup, move |ev| {
        set_clicked(true);
        let point_clicked = (ev.client_x() as f64, ev.client_y() as f64);
        if let Some(_) = document().get_element_by_id("precision_selector") {
            if let Some(selector_pos) = find_element_pos("precision_selector") {
                if point_clicked.0 >= selector_pos.left
                    && point_clicked.0 <= selector_pos.right
                    && point_clicked.1 >= selector_pos.top
                    && point_clicked.1 <= selector_pos.bottom
                {
                    return;
                }
            }
        };

        let (widht, height) = get_window_x_y();
        let (canvs_x, canvas_y) = if is_landscape() {
            (widht / 2., height)
        } else {
            (widht, height / 2.)
        };
        if point_clicked.0 > canvs_x || point_clicked.1 > canvas_y {
            return;
        }

        set_point_clicked(Some(point_clicked));
    });

    let selector_change_handler = move |_| {
        let selected_scenario = document()
            .get_element_by_id("precision_selector")
            .unwrap()
            .dyn_into::<HtmlSelectElement>()
            .unwrap()
            .value()
            .parse::<usize>()
            .unwrap_or(4);
        let precision = selected_scenario.into();

        set_precision.update(|current| *current = precision);
    };

    on_cleanup(move || handler.remove());

    view! {
        <div id="page-container">
            <div id="first" style="margin:2px;flex:1;align-self:stretch;position:relative;padding:5px;border-style:solid;border-color:green;">
            <div style="position:absolute; top: 0; left: 0;"> (-180,90) </div>
            <div style="position:absolute; top: 0; right: 0;"> (180,90) </div>
            <div style="position:absolute; bottom: 0; left: 0;"> (-180,-90) </div>
            <div style="position:absolute; bottom: 0; right: 0;"> (180,-90) </div>
            <div style="position:absolute; bottom: 0; left:0; right:0">
                <select id="precision_selector" style="z-index:10;" on:change=selector_change_handler>
                    <option value="">Select Precision</option>
                    <option value="1">1 characters</option>
                    <option value="2">2 characters</option>
                    <option value="3">3 characters</option>
                    <option value="4">4 characters</option>
                    <option value="5">5 characters</option>
                    <option value="6">6 characters</option>
                    <option value="7">7 characters</option>
                </select>
            </div>
            </div>

            <div id="second" style="flex: 1; height: 100%">
                <Show
                    when=move || clicked()
                    fallback=move || view! {<p> Click in bounded rectangle to find Geo Hash. </p>}
                >
                <p> Describe Geo Hash {clicked_pos()} </p>
                </Show>
            </div>
        </div>
    }
}
