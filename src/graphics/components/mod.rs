use super::PowderState;
use femtovg::{
    renderer::OpenGl, Align, Baseline, Canvas, Color, FillRule, LineCap, LineJoin, Paint, Path,
};
use powder::Meta;

pub(super) fn draw_horizontal_slider(
    canvas: &mut Canvas<OpenGl>,
    x: f32,
    y: f32,
    width: f32,
    min: f32,
    max: f32,
    value: f32,
) {
    let mut paint = Paint::color(Color::rgbf(0.5, 1.0, 0.5));
    paint.set_line_cap(LineCap::Butt);
    paint.set_line_join(LineJoin::Bevel);
    paint.set_line_width(4.0);
    let line_length_value = (((value - min) / (max - min)) * width).clamp(0.0, width);
    let mut path = Path::new();
    path.move_to(x, y);
    path.line_to(x + line_length_value, y);
    canvas.stroke_path(&mut path, paint);
    paint.set_color(Color::rgbf(0.5, 0.5, 0.5));
    let mut path = Path::new();
    path.move_to(x + line_length_value, y);
    path.line_to(x + width, y);
    canvas.stroke_path(&mut path, paint);
}

pub(super) fn draw_button(
    canvas: &mut Canvas<OpenGl>,
    meta: &Meta,
    state: &PowderState,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    label: &str,
) -> bool {
    let mut path = Path::new();
    path.rect(x, y, width, height);

    let is_mouse_hovering = canvas.contains_point(
        &mut path,
        meta.mouse_position.x,
        meta.mouse_position.y,
        FillRule::EvenOdd,
    );

    let mut paint = if is_mouse_hovering {
        Paint::color(Color::rgbf(0.8, 0.8, 0.8))
    } else {
        Paint::color(Color::rgbf(0.95, 0.95, 0.95))
    };
    canvas.fill_path(&mut path, paint);

    paint = Paint::color(Color::rgbf(0.0, 0.0, 0.0));
    paint.set_font_size(24.0);
    paint.set_font(&[state.font.unwrap()]);
    paint.set_text_align(Align::Center);
    paint.set_text_baseline(Baseline::Middle);
    let _ = canvas.fill_text(
        x + width / 2.0,
        y + height / 2.0,
        format!("{}", label),
        paint,
    );

    // Return clicked boolean
    is_mouse_hovering && meta.is_mouse_released
}
