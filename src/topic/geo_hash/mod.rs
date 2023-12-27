use leptos::*;

use crate::graphics::geo_hash::draw_geo_hash;

#[component]
pub fn GeoHashComponent() -> impl IntoView {
    let (clicked, set_clicked) = create_signal(false);
    let (clicked_pos, set_clicked_pos) = create_signal(String::new());
    let handler = window_event_listener(ev::pointerup, move |ev| {
        set_clicked(true);
        let point_clicked = (ev.client_x() as f64, ev.client_y() as f64);
        let (lon, lat, geo_hash) = draw_geo_hash(point_clicked);
        set_clicked_pos(format!(
            "[{}\u{00B0} {}\u{00B0}] {}",
            lon.floor(),
            lat.floor(),
            *geo_hash
        ));
    });
    on_cleanup(move || handler.remove());

    view! {
        <div id="page-container">
            <div id="first" style="margin:2px;flex:1;align-self:stretch;position:relative;padding:5px;border-style:solid;border-color:green;">
            <div style="position:absolute; top: 0; left: 0;"> (-180,90) </div>
            <div style="position:absolute; top: 0; right: 0;"> (180,90) </div>
            <div style="position:absolute; bottom: 0; left: 0;"> (-180,-90) </div>
            <div style="position:absolute; bottom: 0; right: 0;"> (180,-90) </div>
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
