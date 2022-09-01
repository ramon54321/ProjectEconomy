use self::market::Market;
use crate::State;
use accounting::bank::Bank;
use actor::Actor;
use nalgebra_glm::Vec2;
use std::{sync::mpsc::Sender, thread};

pub mod accounting;
pub mod actions;
pub mod actor;
pub mod book;
pub mod market;
pub mod recipe;
pub mod store;

///
/// Runs in own thread. Responsible for simulation.
///
pub fn simulate(tx: Sender<State>) {
    let mut market = Market::new();
    let bank = Bank::new("Federal Reserve");
    let mut actors = vec![
        Actor::new("Actor_1", bank.clone()),
        Actor::new("Actor_2", bank.clone()),
        Actor::new("Actor_3", bank.clone()),
        Actor::new("Actor_4", bank.clone()),
    ];
    loop {
        for actor in actors.iter_mut() {
            actor.tick(&mut market);
        }
        let state = State {
            banks: vec![Vec2::new(1042.0, 481.0), Vec2::new(1012.0, 133.0)],
        };
        tx.send(state).unwrap();
        thread::sleep_ms(1000);
    }
}
