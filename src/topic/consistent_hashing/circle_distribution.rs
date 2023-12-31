use leptos::*;
use wasm_bindgen::JsValue;

use crate::graphics::{canvas_context, clear_canvas, find_element_pos, get_drawable_canvas_x_y};

#[component]
pub(super) fn CircleDistibutionComponent(
    element_id: &'static str,
    server_count: ReadSignal<u32>,
) -> impl IntoView {
    let (widht, height) = get_drawable_canvas_x_y();
    let dom_pos = find_element_pos(element_id).unwrap();
    let center_y = (height - dom_pos.top) / 2. - 20.;
    let center_x = widht / 2. - 20.;
    let radius = center_x.min(center_y);
    let center_x = 10. + center_x;
    let center_y = dom_pos.top + 50. + center_y;
    clear_canvas();
    let context = canvas_context();

    let increment = (2. * std::f64::consts::PI) / 20.;
    for i in 1..21 {
        context.begin_path();
        context.move_to(center_x, center_y);
        let hue_value = format!("hsl({},70%,60%)", i * 15);
        context.set_fill_style(&JsValue::from_str(&hue_value));
        context
            .arc(
                center_x,
                center_y,
                radius,
                i as f64 * increment,
                (i + 1) as f64 * increment,
            )
            .unwrap();
        context.fill();
        context.stroke();
    }

    context.begin_path();
    for i in [0., 20., 40.] {
        context
            .arc(
                center_x,
                center_y,
                radius - i,
                0.0,
                2.0 * std::f64::consts::PI,
            )
            .unwrap();
        context.stroke();
    }
    context.begin_path();
    context.set_fill_style(&JsValue::from_str("#FFFFFF"));
    context
        .arc(
            center_x,
            center_y,
            radius - 40.,
            0.,
            2. * std::f64::consts::PI,
        )
        .unwrap();
    context.fill();

    view! {}
}
