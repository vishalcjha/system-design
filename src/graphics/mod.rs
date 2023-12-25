use std::{cell::RefCell, rc::Rc};

use crate::{
    graphics::{arrow_head::draw_arrowhead, circle_tag::draw_wrapped_number},
    model::{
        arrow::{Arrow, Directional},
        dom_position::DomPosition,
    },
};
mod arrow_head;
mod circle_tag;
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
    // while let Ok(_) = rx.recv_timeout(Duration::from_millis(500)) {}
}

fn draw_line_progressively(arrow: Arrow, progress: f64, batch_number: u32) {
    let context = canvas_context();
    context.set_stroke_style(&JsValue::from_str("#3498db")); // Line color
    context.set_line_width(1.5);

    let ((from_x, from_y), (to_x, to_y)) = arrow.get_from_to();

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
            if arrow.directional != Directional::UnDirectional {
                draw_arrowhead(&context, from_x, from_y, to_x, to_y);
            }

            if arrow.directional == Directional::BiDirectional {
                draw_arrowhead(&context, to_x, to_y, from_x, from_y);
            }

            draw_wrapped_number(
                &context,
                from_x + (to_x - from_x) / 2.,
                from_y + (to_y - from_y) / 2.,
                10.,
                batch_number,
            );
            return;
        }

        current_progress += progress;

        request_animation_frame(f.borrow().as_ref().unwrap());
    }));

    request_animation_frame(g.borrow().as_ref().unwrap());
}

fn canvas_context() -> CanvasRenderingContext2d {
    canvas()
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()
        .unwrap()
}

fn canvas() -> HtmlCanvasElement {
    let canvas = document().get_element_by_id("canvas").unwrap();
    canvas.dyn_into::<web_sys::HtmlCanvasElement>().unwrap()
}

pub fn clear_canvas() {
    let canvas = canvas();
    canvas_context().clear_rect(0., 0., canvas.width() as f64, canvas.height() as f64);
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

pub fn draw_grid_lines(grid_count: u32) {
    let canvas = canvas();
    let height = canvas.height() as f64;
    let widht = canvas.width() as f64 / 2.;

    let context = canvas_context();
    context.set_stroke_style(&JsValue::from_str("#000000")); // Line color
    context.set_line_width(0.5);

    for i in 1..=grid_count {
        context.begin_path();
        context.move_to(0., i as f64 * height / 16.);
        context.line_to(widht, i as f64 * height / 16.);
        context.stroke();

        context.begin_path();
        context.move_to(i as f64 * widht / 16., 0.);
        context.line_to(i as f64 * widht / 16., height);
        context.stroke();
    }
}
