use std::collections::HashSet;

use crate::simulation::store::Store;

use super::{Action, ActionPayload, ActionResult};

pub struct WorkAction {
    step: u8,
}
impl WorkAction {
    pub fn new() -> Self {
        Self { step: 0 }
    }
}

impl Action for WorkAction {
    fn tick(&mut self, payload: ActionPayload) -> ActionResult {
        let task = Task {
            inputs: vec![("Apple".to_string(), 3)],
            outputs: vec![("Food_Packet".to_string(), 1)],
        };

        // Remove listings which no longer exist in the market
        payload
            .submitted_listings
            .retain(|listing| listing.upgrade().is_some());

        // Update target storage
        update_storage_target(payload.store_target, &task, 2.5);

        // Determine how many of each item should be bought or sold
        let storage_deltas = get_storage_deltas(&payload.store_target, &payload.store_actual);

        for delta in storage_deltas.iter() {
            if delta.1 > 0 {
                // Buy
                //let listings_available = payload.market.get_listings_of_kind(&delta.0);
                //if let Some(listing_to_buy) = listings_available.first() {
                //let listing_to_buy_rc = listing_to_buy.upgrade().unwrap();
                //payload.market.unlist_item(listing_to_buy);
                //}
            }
        }

        // Collect input items
        //let input_items = payload.store_actual.take(input, 1);
        //if input_items == 0 {
        //// Get market listings of required item
        //let listings_for_input_item = payload.market.get_listings_of_kind(input.clone());

        //// Purchase cheapest listing
        //// TODO: This does not cost anything, nor does it check amount or success
        //listings_for_input_item
        //.first()
        //.map(|listing| payload.market.unlist_item(listing.clone()));
        //}

        //// Produce output items
        //payload.store_actual.add(output, 1);

        ActionResult::InProgress
    }
    fn get_name(&self) -> String {
        String::from("Idle")
    }
}

type ItemCountList = Vec<(String, isize)>;

struct Task {
    inputs: ItemCountList,
    outputs: ItemCountList,
}

///
/// Mutate the given storage_target store to contain counts which the actor should aim to obtain.
///
fn update_storage_target(storage_target: &mut Store, task: &Task, safety_factor: f32) {
    storage_target.clear();
    for (input_item, input_count) in task.inputs.iter() {
        let amount = ((*input_count as f32) * safety_factor) as isize;
        storage_target.set(&input_item, amount);
    }
}

///
/// Get list of item desire deltas. This list represents how many items the actor has in relation
/// to the target.
///
/// A negative number indicates that the actor has too many of the item.
///
/// The number indicates what the actor must do in order to reach the target.
///
fn get_storage_deltas(storage_target: &Store, storage_actual: &Store) -> ItemCountList {
    let item_kinds_target = storage_target
        .get_item_kinds()
        .into_iter()
        .collect::<HashSet<_>>();
    let item_kinds_actual = storage_actual
        .get_item_kinds()
        .into_iter()
        .collect::<HashSet<_>>();
    let item_kinds = item_kinds_actual.union(&item_kinds_target);
    item_kinds
        .into_iter()
        .map(|item_kind| {
            (
                (*item_kind).clone(),
                (storage_target.count(&item_kind) - storage_actual.count(&item_kind)) as isize,
            )
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_storage_deltas() {
        let mut target = Store::new();
        target.set("Apple", 7);
        target.set("Orange", 3);
        let mut actual = Store::new();
        actual.set("Apple", 5);
        actual.set("Orange", 10);
        let deltas = get_storage_deltas(&target, &actual);
        let apples = deltas.iter().find(|delta| delta.0 == "Apple").unwrap();
        let oranges = deltas.iter().find(|delta| delta.0 == "Orange").unwrap();
        assert_eq!(apples.1, 2);
        assert_eq!(oranges.1, -7);
    }
}