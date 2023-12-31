use std::collections::HashMap;

use leptos::*;
use wasm_bindgen::JsValue;

use crate::graphics::{canvas_context, clear_canvas, find_element_pos, get_drawable_canvas_x_y};

use super::Distribution;

fn draw_stuff(
    element_id: &'static str,
    node_map: HashMap<u32, Vec<u32>>,
    server_map: HashMap<u32, Vec<u32>>,
) {
    leptos::logging::log!("{:?} {:?}", node_map, server_map);
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
    for i in 0..20 {
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

        for (index, j) in [-5., -25.].iter().enumerate() {
            if index == 0 {
                context.set_font("15px Arial");
            } else {
                context.set_font("12px serif");
            }
            let j = *j;
            let which_map = if index == 1 { &server_map } else { &node_map };
            let Some(num) = which_map.get(&(i as u32)) else {
                continue;
            };

            let to_print = if index == 1 {
                format!("{}", num[0] + 1)
            } else {
                format!("{} ", num[0])
            };
            context.set_fill_style(&JsValue::from_str("#000000"));
            context
                .fill_text(
                    &to_print,
                    center_x + (radius + j) * ((i as f64 + 0.5) * increment).cos(),
                    center_y + (radius + j) * ((i as f64 + 0.5) * increment).sin(),
                )
                .unwrap();
        }
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
}

#[component]
pub(super) fn CircleDistibutionComponent(
    element_id: &'static str,
    distribution: ReadSignal<Option<Distribution>>,
) -> impl IntoView {
    create_effect(move |_| {
        if let Some(current) = distribution() {
            let (node_map, server_map) = current;
            let node_map =
                node_map
                    .into_iter()
                    .fold(HashMap::<u32, Vec<u32>>::new(), |mut pos_map, val| {
                        pos_map
                            .entry(val.1)
                            .and_modify(|positions| positions.push(val.0))
                            .or_insert_with(|| vec![val.0]);
                        pos_map
                    });
            let server_map = server_map.into_iter().fold(
                HashMap::<u32, Vec<u32>>::new(),
                |mut server_map, (server_num, positions)| {
                    for pos in positions {
                        server_map
                            .entry(pos)
                            .and_modify(|positions| positions.push(server_num))
                            .or_insert_with(|| vec![server_num]);
                    }
                    server_map
                },
            );
            draw_stuff(element_id, node_map, server_map);
        }
    });

    view! {}
}
