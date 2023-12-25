use web_sys::CanvasRenderingContext2d;

pub(crate) fn draw_wrapped_number(
    context: &CanvasRenderingContext2d,
    center_x: f64,
    center_y: f64,
    radius: f64,
    number: impl ToString,
) {
    draw_circle(context, center_x, center_y, radius);
    draw_number(context, center_x, center_y, number);
}

pub(super) fn draw_circle(
    context: &CanvasRenderingContext2d,
    center_x: f64,
    center_y: f64,
    radius: f64,
) {
    context.begin_path();
    context
        .arc(center_x, center_y, radius, 0.0, 2.0 * std::f64::consts::PI)
        .unwrap();
    context.stroke();
}

fn draw_number(
    context: &CanvasRenderingContext2d,
    center_x: f64,
    center_y: f64,
    number: impl ToString,
) {
    context.set_font("14px Arial"); // Set font size and type
    context.set_text_align("center");
    context.set_text_baseline("middle");

    // Draw the number at the center of the circle
    context
        .fill_text(&number.to_string(), center_x, center_y)
        .unwrap();
}
