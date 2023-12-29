use std::time::Duration;

use leptos::set_timeout_with_handle;

use crate::model::{
    arrow::Directional,
    geo_hash::{GeoHash, Precision},
};

use super::{
    canvas_context, circle_tag::draw_circle, clear_canvas, draw_simple_line, get_window_x_y,
    is_landscape,
};

fn draw_geo_hash_impl(lines: Vec<((f64, f64), (f64, f64))>, loop_count: usize) {
    let (width, height) = get_window_x_y();
    let (width, height) = if is_landscape() {
        (width / 2., height)
    } else {
        (width, height / 2.)
    };

    let progress = 0.04;
    let colors = vec![
        String::from("#3498db"),
        String::from("#db3442"),
        String::from("#34db6c"),
        String::from("#34c8db"),
        String::from("#a3db34"),
        String::from("#c3c9b7"),
        String::from("#16c3de"),
        String::from("#3498db"),
        String::from("#34db6c"),
    ];

    for (lon, lat) in lines.iter().skip(loop_count * 5).take(5) {
        let color = (colors[loop_count]).clone();
        let x_range = get_x_range(lon, width);
        let y_range = get_y_range(lat, height);

        draw_simple_line(
            (x_range.0, y_range.0),
            (x_range.0, y_range.1),
            Directional::UnDirectional,
            progress,
            None,
            Some(color.clone()),
        );

        draw_simple_line(
            (x_range.0, y_range.0),
            (x_range.1, y_range.0),
            Directional::UnDirectional,
            progress,
            None,
            Some(color.clone()),
        );
        draw_simple_line(
            (x_range.1, y_range.1),
            (x_range.1, y_range.0),
            Directional::UnDirectional,
            progress,
            None,
            Some(color.clone()),
        );
        draw_simple_line(
            (x_range.1, y_range.1),
            (x_range.0, y_range.1),
            Directional::UnDirectional,
            progress,
            None,
            Some(color),
        );
    }

    if lines.len() > (loop_count + 1) * 5 {
        let _ = set_timeout_with_handle(
            move || draw_geo_hash_impl(lines, loop_count + 1),
            Duration::from_millis(500),
        );
    }
}

pub fn draw_geo_hash((pos_x, pos_y): (f64, f64), precision: Precision) -> (f64, f64, GeoHash) {
    let (width, height) = get_window_x_y();
    let is_landscape = width > height;
    let (width, height) = if is_landscape {
        (width / 2., height)
    } else {
        (width, height / 2.)
    };

    let lon = (pos_x - (width / 2.)) * 360. / width;
    let lat = ((height / 2.) - pos_y) * 180. / height;

    let geo_hash = GeoHash::new(lat, lon, precision);
    clear_canvas();

    let context = canvas_context();
    draw_circle(&context, pos_x, pos_y, 2.);
    let Some(ref lines) = geo_hash.1 else {
        panic!("this is not to happen");
    };

    draw_geo_hash_impl(lines.clone(), 0);
    // draw_wrapped_number(
    //     &rendering_context,
    //     pos_x,
    //     pos_y,
    //     15.,
    //     format!("{:?}", *geo_hash),
    // );

    (lon, lat, geo_hash)
}

fn get_x_range(lon: &(f64, f64), width: f64) -> (f64, f64) {
    let low = lon.0 + 180.;
    let high = lon.1 + 180.;

    (low * (width / 360.), high * (width / 360.))
}

fn get_y_range(lat: &(f64, f64), height: f64) -> (f64, f64) {
    let low = 90. - lat.0;
    let high = 90. - lat.1;

    (low * (height / 180.), high * (height / 180.))
}
