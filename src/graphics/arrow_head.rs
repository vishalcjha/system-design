use wasm_bindgen::JsValue;
use web_sys::CanvasRenderingContext2d;

pub(super) fn draw_arrowhead(
    context: &CanvasRenderingContext2d,
    x1: f64,
    y1: f64,
    x2: f64,
    y2: f64,
) {
    let arrow_size = 5.0; // Adjust arrowhead size

    // Calculate the angle between the line and the x-axis
    let angle = (y2 - y1).atan2(x2 - x1);

    // Calculate arrowhead points
    let arrow_x1 = x2; //- arrow_size * angle.cos();
    let arrow_y1 = y2; //- arrow_size * angle.sin();
    let arrow_x2 = x2 - arrow_size * angle.cos() - arrow_size * 0.5 * angle.sin();
    let arrow_y2 = y2 - arrow_size * angle.sin() + arrow_size * 0.5 * angle.cos();
    let arrow_x3 = x2 - arrow_size * angle.cos() + arrow_size * 0.5 * angle.sin();
    let arrow_y3 = y2 - arrow_size * angle.sin() - arrow_size * 0.5 * angle.cos();

    context.set_fill_style(&JsValue::from_str("#3498db"));
    // Draw the arrowhead
    context.begin_path();
    context.move_to(arrow_x1, arrow_y1);
    context.line_to(arrow_x2, arrow_y2);
    context.line_to(arrow_x3, arrow_y3);
    context.close_path();
    context.fill();
}
