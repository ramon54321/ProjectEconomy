use accounting::bank::Bank;
use actor::Actor;
use std::thread;

pub mod accounting;
pub mod actions;
pub mod actor;
pub mod book;
pub mod recipe;
pub mod store;

///
/// Runs in own thread. Responsible for simulation.
///
pub fn simulate() {
    let bank = Bank::new("Federal Reserve");
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
}
