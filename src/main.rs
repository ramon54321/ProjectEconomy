use economy::accounting::bank::*;
use economy::actor::Actor;

use femtovg::{Color, LineCap, LineJoin, Paint, Path};
use nalgebra_glm::Vec2;
use powder::Powder;
use std::thread;

fn main() {
    //let account_a = bank.borrow_mut().open_account(bank.clone(), "Johnny");
    //let account_b = bank.borrow_mut().open_account(bank.clone(), "Jill");

    //bank.borrow_mut()
    //.transfer(account_a.clone(), account_b.clone(), 500);
    //println!("{:?}", account_a.borrow().bank.upgrade().unwrap().borrow());

    let compute_thread = thread::spawn(|| {
        let bank = FederalReserve::new("Washington");
        let mut actors = vec![
            Actor::new("Actor_1", bank.clone()),
            Actor::new("Actor_2", bank.clone()),
            Actor::new("Actor_3", bank.clone()),
            Actor::new("Actor_4", bank.clone()),
        ];
        loop {
            for actor in actors.iter_mut() {
                actor.tick();
            }
            thread::sleep_ms(1000);
        }
    });

    // Graphical Rendering
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
