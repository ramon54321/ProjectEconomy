use self::{
    market::{item::Item, Market},
    task::Task,
};
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
pub mod item_count_list;
pub mod logbook;
pub mod market;
pub mod recipe;
pub mod store;
pub mod task;

///
/// Runs in own thread. Responsible for simulation.
///
pub fn simulate(tx: Sender<RenderableState>) {
    let mut rng = thread_rng();

    // Setup Simulation
    let mut market = Market::new();
    let bank = Bank::new("Federal Reserve");
    let task_farmer = Task {
        inputs: vec![],
        outputs: vec![("Apple".to_string(), 1)],
    };
    let task_packer = Task {
        inputs: vec![("Apple".to_string(), 3)],
        outputs: vec![("FoodBox".to_string(), 1)],
    };
    let mut actors = vec![
        Actor::new("Actor_1", bank.clone(), task_farmer.clone()),
        Actor::new("Actor_2", bank.clone(), task_farmer.clone()),
        Actor::new("Actor_3", bank.clone(), task_packer.clone()),
        Actor::new("Actor_4", bank.clone(), task_packer.clone()),
    ];

    // TODO: Render all actors and see if you can make them sell apples and buy apples and sell
    // foodboxes

    // Run Simulation
    loop {
        // Tick each actor
        for actor in actors.iter_mut() {
            actor.borrow_mut().tick(&mut market);
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

        let actor_logs = actors
            .iter()
            .map(|actor| (actor.borrow().get_name(), actor.borrow().get_log()))
            .collect::<Vec<_>>();

        let renderable_state = RenderableState {
            actor_count: actors.len(),
            listed_item_kinds,
            actor_logs,
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
