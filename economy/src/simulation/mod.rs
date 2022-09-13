use self::market::{item::Item, Market};
use crate::RenderableState;
use accounting::bank::Bank;
use actor::Actor;
use rand::{random, seq::SliceRandom, thread_rng, Rng};
use std::{sync::mpsc::Sender, thread, time::Duration};
use uuid::Uuid;

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
pub fn simulate(tx: Sender<RenderableState>) {
    let mut rng = thread_rng();

    // Setup Simulation
    let mut market = Market::new();
    let bank = Bank::new("Federal Reserve");
    let mut actors = vec![
        Actor::new("Actor_1", bank.clone()),
        Actor::new("Actor_2", bank.clone()),
        Actor::new("Actor_3", bank.clone()),
        Actor::new("Actor_4", bank.clone()),
    ];
    for _ in 0..rng.gen_range(30..60) {
        market.list_item(create_random_item(), rng.gen_range(10..150));
    }

    // Run Simulation
    loop {
        // Tick each actor
        for actor in actors.iter_mut() {
            actor.tick(&mut market);
        }

        // Build renderable state
        // TODO: Render these items and their counts - This will show what the market is like -
        // Perhaps also render their average price?
        let listed_item_kinds = market
            .get_listed_item_kinds()
            .map(|item_kind| {
                (
                    item_kind.clone(),
                    market.get_listings_of_kind(item_kind).len(),
                )
            })
            .collect::<Vec<_>>();
        let renderable_state = RenderableState {
            actor_count: actors.len(),
            listed_item_kinds,
        };

        // Send renderable state through channel to be rendered
        tx.send(renderable_state).unwrap();

        // Wait for next tick
        thread::sleep(Duration::from_millis(1000));
    }
}

// Helpers
fn create_random_item() -> Item {
    let kinds = vec!["Apple", "Orange", "Banana"];
    Item {
        id: Uuid::new_v4(),
        kind: kinds.choose(&mut thread_rng()).unwrap().to_string(),
    }
}
