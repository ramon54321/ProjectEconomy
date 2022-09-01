use crate::RenderableState;
use femtovg::{Baseline, Color, LineCap, LineJoin, Paint, Path};
use nalgebra_glm::Vec2;
use powder::Powder;
use std::sync::mpsc::Receiver;

pub fn render(rx: Receiver<RenderableState>) {
    let initial_state = RenderableState {
        banks: vec![Vec2::new(342.0, 481.0), Vec2::new(612.0, 133.0)],
    };
    let mut powder = Powder::new(initial_state).expect("Could not start powder");
    let roboto_regular = powder.load_font("assets/Roboto-Regular.ttf");
    powder.push(Box::new(move |canvas, meta, state| {
        match rx.try_recv() {
            Ok(new_state) => *state = new_state,
            _ => (),
        };
    }));
    powder.push(Box::new(move |canvas, meta, state| {
        let mut paint = Paint::color(Color::rgbf(1.0, 1.0, 1.0));
        paint.set_line_cap(LineCap::Butt);
        paint.set_line_join(LineJoin::Bevel);
        paint.set_line_width(1.0);

        let mut path = Path::new();
        for bank in state.banks.iter() {
            path.circle(bank.x, bank.y, 10.0);
        }

        let mut text_paint = Paint::color(Color::rgba(255, 255, 255, 128));
        text_paint.set_font_size(28.0);
        text_paint.set_font(&[roboto_regular]);
        //text_paint.set_text_align(Align::Left);
        text_paint.set_text_baseline(Baseline::Middle);
        let _ = canvas.fill_text(50.0, 50.0, "Hello, world!", text_paint);

        canvas.fill_path(&mut path, paint);
    }));
    powder.start();
}
