use self::{item::Item, listing::Listing};
use super::{accounting::account::Account, actor::Actor};
use std::{
    cell::RefCell,
    collections::HashMap,
    rc::{Rc, Weak},
};
use uuid::Uuid;

pub mod item;
pub mod listing;

pub struct Market {
    listings_by_id: HashMap<Uuid, Rc<Listing>>,
    listings_by_item_kind: HashMap<String, Vec<Weak<Listing>>>,
    listings_by_owner_id: HashMap<Uuid, Vec<Weak<Listing>>>,
}
impl Market {
    pub(super) fn new() -> Self {
        Self {
            listings_by_id: HashMap::new(),
            listings_by_item_kind: HashMap::new(),
            listings_by_owner_id: HashMap::new(),
        }
    }
    ///
    /// List item on market at a given price. Items that are listed are able to be unlisted.
    ///
    pub(super) fn list_item(
        &mut self,
        owner: Option<Weak<RefCell<Actor>>>,
        item: Item,
        price: i64,
    ) -> Weak<Listing> {
        let listing = Rc::new(Listing::new(owner.clone(), item.clone(), price));
        let uuid = listing.id;

        // Add main listing
        self.listings_by_id.insert(uuid.clone(), listing.clone());

        // Add to item kind index
        if !self.listings_by_item_kind.contains_key(&item.kind) {
            self.listings_by_item_kind
                .insert(item.kind.clone(), vec![Rc::downgrade(&listing)]);
        } else {
            self.listings_by_item_kind
                .get_mut(&item.kind)
                .unwrap()
                .push(Rc::downgrade(&listing));
        }

        // Add to owner id index
        if let Some(owner) = owner {
            let owner_id = owner.upgrade().unwrap().borrow().id;
            if !self.listings_by_owner_id.contains_key(&owner_id) {
                self.listings_by_owner_id
                    .insert(owner_id.clone(), vec![Rc::downgrade(&listing)]);
            } else {
                self.listings_by_owner_id
                    .get_mut(&owner_id)
                    .unwrap()
                    .push(Rc::downgrade(&listing));
            }
        }

        Rc::downgrade(&listing)
    }
    ///
    /// Unlist an item from the market given a Weak reference to the item. If the Weak reference
    /// can not be upgraded, no action will be taken.
    ///
    pub(super) fn unlist_item(&mut self, listing: Weak<Listing>) {
        let listing = listing.upgrade();
        if listing.is_none() {
            return;
        }
        let listing = listing.unwrap();

        // Ensure listing exists in market
        if !self.listings_by_id.contains_key(&listing.id) {
            return;
        }

        // Remove main listing
        self.listings_by_id.remove(&listing.id);

        // Remove listing from item kind index
        self.listings_by_item_kind
            .get_mut(&listing.item.kind)
            .unwrap()
            .retain(|l| l.upgrade().unwrap().id != listing.id);

        // Remove listing from owner id index
        if let Some(owner) = listing.owner.as_ref().and_then(|owner| owner.upgrade()) {
            self.listings_by_owner_id
                .get_mut(&owner.borrow().id)
                .unwrap()
                .retain(|l| l.upgrade().unwrap().id != listing.id);
        }
    }
    ///
    /// Get an iterator over each item kind present in the listing map.
    ///
    pub(super) fn get_listed_item_kinds(&self) -> impl Iterator<Item = &String> {
        self.listings_by_item_kind.keys()
    }
    ///
    /// Get a Vec of Weak references to listings for items of the given kind. There are no
    /// guarentees the Weak references stay valid at any point after the call to this method.
    ///
    pub(super) fn get_listings_of_kind(&self, kind: &str) -> Vec<Weak<Listing>> {
        match self.listings_by_item_kind.get(kind) {
            None => Vec::new(),
            Some(listings) => {
                let mut sorted = listings.clone();
                sorted.sort_by(|a, b| a.upgrade().unwrap().price.cmp(&b.upgrade().unwrap().price));
                sorted
            }
        }
    }
    ///
    /// Removes the listing from the market and moves payment from buyer to seller
    ///
    pub(super) fn buy_listing(listing: Weak<Listing>, buyer_account: &mut Account) {
        //buyer_account.get_bank().upgrade().unwrap().borrow().
    }
}

#[cfg(test)]
mod tests {
    use crate::simulation::accounting::bank::Bank;

    use super::*;
    use uuid::Uuid;

    #[test]
    fn list_item() {
        let mut market = Market::new();
        market.list_item(
            None,
            Item {
                id: Uuid::new_v4(),
                kind: "ABC".to_string(),
            },
            500,
        );
        assert_eq!(market.listings_by_item_kind.len(), 1);
        assert_eq!(
            market
                .listings_by_item_kind
                .get("ABC")
                .unwrap()
                .first()
                .unwrap()
                .upgrade()
                .unwrap()
                .price,
            500
        );
    }

    #[test]
    fn get_listings_of_kind() {
        let mut market = Market::new();
        market.list_item(
            None,
            Item {
                id: Uuid::new_v4(),
                kind: "ABC".to_string(),
            },
            500,
        );
        market.list_item(
            None,
            Item {
                id: Uuid::new_v4(),
                kind: "ABC".to_string(),
            },
            750,
        );
        market.list_item(
            None,
            Item {
                id: Uuid::new_v4(),
                kind: "DEF".to_string(),
            },
            250,
        );
        // Two different kinds of listings
        assert_eq!(market.listings_by_item_kind.len(), 2);
        // Two listings of kind 'ABC'
        assert_eq!(market.listings_by_item_kind.get("ABC").unwrap().len(), 2);
        // Get listings of specific kind
        let abcs = market.get_listings_of_kind("ABC");
        assert_eq!(abcs.len(), 2);
        // Get listings of non-existing item kind
        let should_be_empty = market.get_listings_of_kind("GHI");
        assert_eq!(should_be_empty.len(), 0);
    }

    #[test]
    fn get_listings_of_owner() {
        let bank = Bank::new("Bank");
        let owner_a = Actor::new("A", bank.clone());
        let owner_b = Actor::new("B", bank.clone());
        let mut market = Market::new();
        market.list_item(
            Some(Rc::downgrade(&owner_a)),
            Item {
                id: Uuid::new_v4(),
                kind: "ABC".to_string(),
            },
            500,
        );
        market.list_item(
            Some(Rc::downgrade(&owner_b)),
            Item {
                id: Uuid::new_v4(),
                kind: "ABC".to_string(),
            },
            750,
        );
        market.list_item(
            Some(Rc::downgrade(&owner_a)),
            Item {
                id: Uuid::new_v4(),
                kind: "DEF".to_string(),
            },
            250,
        );
        assert_eq!(market.listings_by_owner_id.len(), 2);
        assert_eq!(
            market
                .listings_by_owner_id
                .get(&owner_a.borrow().id)
                .unwrap()
                .len(),
            2
        );
    }

    #[test]
    fn unlist_item() {
        let mut market = Market::new();
        market.list_item(
            None,
            Item {
                id: Uuid::new_v4(),
                kind: "ABC".to_string(),
            },
            500,
        );
        market.list_item(
            None,
            Item {
                id: Uuid::new_v4(),
                kind: "ABC".to_string(),
            },
            750,
        );
        assert_eq!(market.get_listings_of_kind("ABC").len(), 2);
        let abcs = market.get_listings_of_kind("ABC");
        let chosen_listing = abcs
            .iter()
            .find(|listing| listing.upgrade().unwrap().price == 500)
            .unwrap()
            .clone();
        market.unlist_item(chosen_listing);
        assert_eq!(market.get_listings_of_kind("ABC").len(), 1);
    }
}
