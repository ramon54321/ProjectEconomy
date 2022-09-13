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
        // Construct input and output requirements from recipe
        let recipe = String::from("Apple-Food_Packet");
        let mut recipe_split = recipe.split("-");
        let input = recipe_split.next().unwrap();
        let output = recipe_split.next().unwrap();

        // Collect input items
        let input_items = payload.store_actual.take(input, 1);
        if input_items == 0 {
            // Get market listings of required item
            let listings_for_input_item = payload.market.get_listings_of_kind(input.clone());

            // Purchase cheapest listing
            // TODO: This does not cost anything, nor does it check amount or success
            listings_for_input_item
                .first()
                .map(|listing| payload.market.unlist_item(listing.clone()));
        }

        // Produce output items
        payload.store_actual.add(output, 1);

        ActionResult::InProgress
    }
    fn get_name(&self) -> String {
        String::from("Idle")
    }
}

struct OccupationSpec {
    //needs: Vec<(String, usize)>,
    recipe: String,
}
