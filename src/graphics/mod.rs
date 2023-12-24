use std::{cell::RefCell, rc::Rc};

use crate::model::{arrow::Arrow, dom_position::DomPosition};
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

pub(crate) fn draw_line(arrow: &Arrow) {
    draw_line_progressively(arrow.clone(), 0.2);
}

fn draw_line_progressively(arrow: Arrow, progress: f64) {
    let canvas = canvas();
    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()
        .unwrap();
    context.set_stroke_style(&JsValue::from_str("#3498db")); // Line color
    context.set_line_width(3.0);

    let ((from_x, from_y), (to_x, to_y)) = arrow.get_from_to();

    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    let mut progress = progress;
    *g.borrow_mut() = Some(Closure::new(move || {
        let to_x = from_x + (to_x - from_x) * progress;
        let to_y = from_y + (to_y - from_y) * progress;

        context.begin_path();
        context.move_to(from_x, from_y);
        context.line_to(to_x, to_y);
        context.stroke();
        if progress >= 1. {
            let _ = f.borrow_mut().take();
            return;
        }

        progress += 0.03;

        request_animation_frame(f.borrow().as_ref().unwrap());
    }));

    request_animation_frame(g.borrow().as_ref().unwrap());
}

fn canvas() -> HtmlCanvasElement {
    let canvas = document().get_element_by_id("canvas").unwrap();
    canvas.dyn_into::<web_sys::HtmlCanvasElement>().unwrap()
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}
