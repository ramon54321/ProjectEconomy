use super::{Action, ActionPayload, ActionResult};
use crate::simulation::{
    item_count_list::ItemCountList, market::item::Item, store::Store, task::Task,
};
use std::collections::HashSet;
use uuid::Uuid;

pub struct WorkAction {
    has_used_material: bool,
    progress_points: u64,
}
impl WorkAction {
    pub fn new() -> Self {
        Self {
            has_used_material: false,
            progress_points: 0,
        }
    }
}

impl Action for WorkAction {
    fn tick(&mut self, payload: ActionPayload) -> ActionResult {
        // Break to different action if there is no longer a task
        if payload.task.is_none() {
            return ActionResult::InProgress;
        }
        let task = payload.task.clone().unwrap();

        // Remove listings which no longer exist in the market
        payload
            .submitted_listings
            .retain(|listing| listing.upgrade().is_some());

        // Update target storage
        update_storage_target(payload.store_target, &task, 2.5);

        // Determine how many of each item should be bought or sold
        let storage_deltas = get_storage_deltas(&payload.store_target, &payload.store_actual);

        // Try to sequentially trade deltas
        for delta in storage_deltas.iter() {
            let item_kind = delta.0.clone();
            let amount = delta.1;
            if amount > 0 {
                // Buy
                payload
                    .log
                    .add_entry(&format!("Need to buy {} of {}", amount, item_kind));
                let listings_available = payload.market.get_listings_of_kind(&item_kind);
                if let Some(listing_to_buy) = listings_available.first() {
                    let listing_to_buy_strong = listing_to_buy.upgrade();
                    if listing_to_buy_strong.is_none() {
                        continue;
                    }
                    let successful_buy = payload
                        .market
                        .buy_listing(listing_to_buy.clone(), payload.account.clone());
                    if successful_buy {
                        payload.store_actual.add(&item_kind, 1);
                    }
                }
            } else if amount < 0 {
                // Check if actor already listed item
                let existing_listings_for_item_kind_count = payload
                    .submitted_listings
                    .iter()
                    .filter(|listing| {
                        // Safe becaue listings are cleaned up before tick
                        let listing = listing
                            .upgrade()
                            .expect("Could not upgrade listing, should be cleaned up before tick");
                        listing.item.kind == item_kind
                    })
                    .count() as isize;

                let amount_to_list = (-amount) - existing_listings_for_item_kind_count;

                if amount_to_list > 0 {
                    // List
                    payload
                        .log
                        .add_entry(&format!("Need to list {} of {}", amount_to_list, item_kind));

                    // Remove the amount to list from the store
                    payload.store_actual.take(&item_kind, amount_to_list);

                    // Add listing for each item to market
                    for _ in 0..amount_to_list {
                        // List item in market
                        let listing = payload.market.list_item(
                            Some(payload.actor_weak.clone()),
                            Item {
                                id: Uuid::new_v4(),
                                kind: item_kind.clone(),
                            },
                            500,
                        );

                        // Record listing on actor
                        payload.submitted_listings.push(listing);
                    }
                }
            }
        }

        // Ensure material usage
        if !self.has_used_material {
            let has_enough_material = task
                .inputs
                .iter()
                .all(|input| payload.store_actual.has_count(&input.0, input.1));

            if !has_enough_material {
                payload
                    .log
                    .add_entry(&format!("Does not have enough material yet...",));
                return ActionResult::InProgress;
            }

            // Remove items from storage
            for (input_item_kind, input_item_needed_count) in task.inputs.iter() {
                payload
                    .store_actual
                    .take(&input_item_kind, *input_item_needed_count);
            }

            self.has_used_material = true;
        }

        self.progress_points = self.progress_points + 4;
        payload.log.add_entry(&format!(
            "Working progress at {} points",
            self.progress_points
        ));

        if self.progress_points > task.work_points {
            // Produce output
            for (output_item_kind, output_item_count) in task.outputs.iter() {
                payload
                    .store_actual
                    .add(&output_item_kind, *output_item_count);
            }

            return ActionResult::Done(Box::new(WorkAction::new()));
        }

        ActionResult::InProgress
    }
    fn get_name(&self) -> String {
        String::from("Idle")
    }
}

///
/// Mutate the given storage_target store to contain counts which the actor should aim to obtain.
///
/// This strategy simply tries to store what the task needs as input multiplied by a safety factor.
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
