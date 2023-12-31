use std::{cell::RefCell, rc::Rc};

use crate::{
    graphics::{arrow_head::draw_arrowhead, circle_tag::draw_wrapped_number},
    model::{
        arrow::{Arrow, Directional},
        dom_position::DomPosition,
    },
};
mod arrow_head;
pub(crate) mod circle_tag;
pub(crate) mod geo_hash;
use leptos::{document, window};
use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

pub(crate) fn find_element_pos(element_id: impl AsRef<str>) -> Option<DomPosition> {
    let element = document().get_element_by_id(element_id.as_ref())?;
    let dom_rect = element.get_bounding_client_rect();
    let (top, left, bottom, right) = (
        dom_rect.top(),
        dom_rect.left(),
        dom_rect.bottom(),
        dom_rect.right(),
    );
    Some(DomPosition::new(top, left, bottom, right))
}

pub(crate) fn draw_lines_concurrently(arrows: Vec<&Arrow>, batch_number: u32) {
    for arrow in arrows {
        draw_line_progressively(arrow.clone(), 0.03, batch_number);
    }
}
pub(self) fn draw_simple_line(
    (from_x, from_y): (f64, f64),
    (to_x, to_y): (f64, f64),
    directional: Directional,
    progress: f64,
    batch_number: Option<u32>,
    color: Option<String>,
) {
    let context = canvas_context();
    let default_color = String::from("#3498db");
    context.set_stroke_style(&JsValue::from_str(
        (color.unwrap_or(default_color)).as_ref(),
    )); // Line color
    context.set_line_width(1.5);

    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    let mut current_progress = progress;

    *g.borrow_mut() = Some(Closure::new(move || {
        let to_x = from_x + (to_x - from_x) * current_progress;
        let to_y = from_y + (to_y - from_y) * current_progress;

        context.begin_path();
        context.move_to(from_x, from_y);
        context.line_to(to_x, to_y);
        context.stroke();

        if current_progress >= 1. {
            let _ = f.borrow_mut().take();
            if directional != Directional::UnDirectional {
                draw_arrowhead(&context, from_x, from_y, to_x, to_y);
            }

            if directional == Directional::BiDirectional {
                draw_arrowhead(&context, to_x, to_y, from_x, from_y);
            }

            if let Some(batch_number) = batch_number {
                draw_wrapped_number(
                    from_x + (to_x - from_x) / 2.,
                    from_y + (to_y - from_y) / 2.,
                    10.,
                    batch_number,
                );
            }

            return;
        }

        current_progress += progress;

        request_animation_frame_custom(f.borrow().as_ref().unwrap());
    }));

    request_animation_frame_custom(g.borrow().as_ref().unwrap());
}
fn draw_line_progressively(arrow: Arrow, progress: f64, batch_number: u32) {
    let ((from_x, from_y), (to_x, to_y)) = arrow.get_from_to();

    draw_simple_line(
        (from_x, from_y),
        (to_x, to_y),
        arrow.directional.clone(),
        progress,
        Some(batch_number),
        None,
    );
}

pub(crate) fn canvas_context() -> CanvasRenderingContext2d {
    canvas()
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()
        .unwrap()
}

pub fn canvas() -> HtmlCanvasElement {
    let canvas = document().get_element_by_id("canvas").unwrap();
    canvas.dyn_into::<web_sys::HtmlCanvasElement>().unwrap()
}

pub fn clear_canvas() {
    let canvas = canvas();
    canvas_context().clear_rect(0., 0., canvas.width() as f64, canvas.height() as f64);
    let width = window().inner_width().unwrap().as_f64().unwrap() as u32;
    let height = window().inner_height().unwrap().as_f64().unwrap() as u32;
    canvas.set_width(width);
    canvas.set_height(height);
}

pub fn request_animation_frame_custom(f: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

pub fn draw_grid_lines(grid_count: u32, is_landscape: bool) {
    let canvas = canvas();
    let height = canvas.height() as f64;
    let widht = canvas.width() as f64;
    let (width, height) = if is_landscape {
        (widht / 2., height)
    } else {
        (widht, height / 2.)
    };

    let context = canvas_context();
    context.set_stroke_style(&JsValue::from_str("#000000")); // Line color
    context.set_line_width(0.5);

    for i in 1..=grid_count {
        context.begin_path();
        context.move_to(0., i as f64 * height / 16.);
        context.line_to(width, i as f64 * height / 16.);
        context.stroke();

        context.begin_path();
        context.move_to(i as f64 * width / 16., 0.);
        context.line_to(i as f64 * width / 16., height);
        context.stroke();
    }
}

pub(super) fn get_window_x_y() -> (f64, f64) {
    (
        window().inner_width().unwrap().as_f64().unwrap(),
        window().inner_height().unwrap().as_f64().unwrap(),
    )
}

pub(super) fn get_drawable_canvas_x_y() -> (f64, f64) {
    let (width, height) = get_window_x_y();
    if is_landscape() {
        (width / 2., height)
    } else {
        (width, height / 2.)
    }
}

pub(super) fn is_landscape() -> bool {
    let (width, height) = get_window_x_y();
    width > height
}
