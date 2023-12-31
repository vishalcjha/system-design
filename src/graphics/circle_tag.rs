use super::canvas_context;

pub(crate) fn draw_wrapped_number(
    center_x: f64,
    center_y: f64,
    radius: f64,
    number: impl ToString,
) {
    draw_circle(center_x, center_y, radius);
    draw_number(center_x, center_y, number);
}

pub(super) fn draw_circle(center_x: f64, center_y: f64, radius: f64) {
    let context = canvas_context();
    context.begin_path();
    context
        .arc(center_x, center_y, radius, 0.0, 2.0 * std::f64::consts::PI)
        .unwrap();
    context.stroke();
}

fn draw_number(center_x: f64, center_y: f64, number: impl ToString) {
    let context = canvas_context();
    context.set_font("14px Arial"); // Set font size and type
    context.set_text_align("center");
    context.set_text_baseline("middle");

    // Draw the number at the center of the circle
    context
        .fill_text(&number.to_string(), center_x, center_y)
        .unwrap();
}
