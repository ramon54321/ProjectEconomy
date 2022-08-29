use economy::simulation::simulate;
use femtovg::{Color, LineCap, LineJoin, Paint, Path};
use nalgebra_glm::Vec2;
use powder::Powder;
use std::thread;

fn main() {
    // Simulation
    let simulation_thread = thread::spawn(simulate);

    // Graphics
    let state = State {
        banks: vec![Vec2::new(342.0, 481.0), Vec2::new(612.0, 133.0)],
    };

    let mut powder = Powder::new(state).expect("Could not start powder");
    powder.push(Box::new(|canvas, meta, state| {
        let mut paint = Paint::color(Color::rgbf(1.0, 1.0, 1.0));
        paint.set_line_cap(LineCap::Butt);
        paint.set_line_join(LineJoin::Bevel);
        paint.set_line_width(1.0);

        let mut path = Path::new();
        for bank in state.banks.iter() {
            path.circle(bank.x, bank.y, 10.0);
        }
        canvas.fill_path(&mut path, paint);
    }));
    powder.start();
}

struct State {
    banks: Vec<Vec2>,
}
