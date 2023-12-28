use leptos::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlSelectElement;

use crate::{graphics::geo_hash::draw_geo_hash, model::geo_hash::Precision};

#[component]
pub fn GeoHashComponent() -> impl IntoView {
    let (clicked, set_clicked) = create_signal(false);
    let (clicked_pos, set_clicked_pos) = create_signal(String::new());
    let (precision, set_precision) = create_signal(Precision::FOUR);
    let handler = window_event_listener(ev::pointerup, move |ev| {
        set_clicked(true);
        let point_clicked = (ev.client_x() as f64, ev.client_y() as f64);
        let (lon, lat, geo_hash) = draw_geo_hash(point_clicked, precision());
        set_clicked_pos(format!(
            "[{}\u{00B0} {}\u{00B0}] {}",
            lon.floor(),
            lat.floor(),
            *geo_hash
        ));
    });

    let selector_change_handler = move |_| {
        let selected_scenario = document()
            .get_element_by_id("precision_selector")
            .unwrap()
            .dyn_into::<HtmlSelectElement>()
            .unwrap()
            .value()
            .parse::<u32>()
            .unwrap_or(4);
        use Precision::*;
        let precision = match selected_scenario {
            5 => FIVE,
            6 => SIX,
            7 => SEVEN,
            _ => FOUR,
        };

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
