use crate::graphics::PowderState;
use femtovg::{renderer::OpenGl, Align, Baseline, Canvas, Color, Paint};
use powder::Meta;

pub fn render_basic_info(canvas: &mut Canvas<OpenGl>, meta: &mut Meta, state: &mut PowderState) {
    let mut paint = Paint::color(Color::rgbf(1.0, 1.0, 1.0));
    paint.set_font_size(28.0);
    paint.set_font(&[state.font.unwrap()]);
    paint.set_text_align(Align::Left);
    paint.set_text_baseline(Baseline::Top);
    let _ = canvas.fill_text(10.0, 5.0, format!("FPS: {}", meta.frames_per_second), paint);
    let _ = canvas.fill_text(
        10.0,
        35.0,
        format!("Actors: {}", state.renderable_state.actor_count),
        paint,
    );
}
