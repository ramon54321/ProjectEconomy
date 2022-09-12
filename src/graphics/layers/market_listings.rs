use crate::graphics::PowderState;
use femtovg::{renderer::OpenGl, Align, Baseline, Canvas, Color, LineCap, LineJoin, Paint, Path};
use powder::Meta;

pub fn render_market_listings(
    canvas: &mut Canvas<OpenGl>,
    _meta: &mut Meta,
    state: &mut PowderState,
) {
    let mut paint = Paint::color(Color::rgbf(1.0, 1.0, 1.0));
    paint.set_line_cap(LineCap::Butt);
    paint.set_line_join(LineJoin::Bevel);
    paint.set_line_width(4.0);
    paint.set_font_size(14.0);
    paint.set_font(&[state.font.unwrap()]);
    paint.set_text_align(Align::Right);
    paint.set_text_baseline(Baseline::Middle);

    let maximum_count = state
        .renderable_state
        .listed_item_kinds
        .iter()
        .reduce(|acc, cur| if cur.1 > acc.1 { cur } else { acc })
        .map(|res| res.1);
    if maximum_count.is_none() {
        return;
    }
    let maximum_count = maximum_count.unwrap();

    for (i, (item_kind, count)) in state.renderable_state.listed_item_kinds.iter().enumerate() {
        let line_length = ((*count as f32) / (maximum_count as f32)) * 800.0;
        let vertical_offset = 50.0 + i as f32 * 30.0;
        let _ = canvas.fill_text(250.0, vertical_offset, format!("{}", item_kind), paint);
        let mut path = Path::new();
        path.move_to(300.0, vertical_offset);
        path.line_to(300.0 + line_length, vertical_offset);
        canvas.stroke_path(&mut path, paint);
    }
}
