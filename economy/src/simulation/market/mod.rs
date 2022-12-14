use self::{item::Item, listing::Listing};
use super::{
    accounting::{account::Account, bank::Bank},
    actor::Actor,
};
use std::{
    cell::RefCell,
    collections::{HashMap, VecDeque},
    rc::{Rc, Weak},
};
use uuid::Uuid;

pub mod item;
pub mod listing;

pub struct Market {
    listings_by_id: HashMap<Uuid, Rc<Listing>>,
    listings_by_item_kind: HashMap<String, Vec<Weak<Listing>>>,
    listings_by_owner_id: HashMap<Uuid, Vec<Weak<Listing>>>,
    listing_queue: VecDeque<Rc<Listing>>,
}
impl Market {
    pub(super) fn new() -> Self {
        Self {
            listings_by_id: HashMap::new(),
            listings_by_item_kind: HashMap::new(),
            listings_by_owner_id: HashMap::new(),
            listing_queue: VecDeque::new(),
        }
    }
    ///
    /// Processes all enqueued listings
    ///
    pub(super) fn tick(&mut self) {
        while let Some(pending_listing) = self.listing_queue.pop_front() {
            let uuid = pending_listing.id;

            // Add main listing
            self.listings_by_id
                .insert(uuid.clone(), pending_listing.clone());

            // Add to item kind index
            if !self
                .listings_by_item_kind
                .contains_key(&pending_listing.item.kind)
            {
                self.listings_by_item_kind.insert(
                    pending_listing.item.kind.clone(),
                    vec![Rc::downgrade(&pending_listing)],
                );
            } else {
                self.listings_by_item_kind
                    .get_mut(&pending_listing.item.kind)
                    .unwrap()
                    .push(Rc::downgrade(&pending_listing));
            }

            // Add to owner id index
            if let Some(owner) = pending_listing
                .owner
                .as_ref()
                .and_then(|owner| owner.upgrade())
            {
                if !self.listings_by_owner_id.contains_key(&owner.borrow().id) {
                    self.listings_by_owner_id
                        .insert(owner.borrow().id, vec![Rc::downgrade(&pending_listing)]);
                } else {
                    self.listings_by_owner_id
                        .get_mut(&owner.borrow().id)
                        .unwrap()
                        .push(Rc::downgrade(&pending_listing));
                }
            }
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

        // Enque listing
        self.listing_queue.push_back(listing.clone());

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
    pub(super) fn buy_listing(
        &mut self,
        listing: Weak<Listing>,
        buyer_account: Weak<RefCell<Account>>,
    ) -> bool {
        // Ensure valid listing
        let listing = listing.upgrade();
        if listing.is_none() {
            return false;
        }
        let listing = listing.unwrap();

        // Ensure valid seller account
        let seller_account = listing
            .owner
            .clone()
            .and_then(|owner| owner.upgrade())
            .and_then(|owner| Some(owner.borrow().get_account()));
        if seller_account.is_none() {
            return false;
        }
        let seller_account = seller_account.unwrap();

        // Process transation in applicable direction
        let amount = listing.price;
        if amount >= 0 {
            Bank::process_transaction(buyer_account, seller_account, amount as u64);
        } else {
            Bank::process_transaction(seller_account, buyer_account, -amount as u64);
        }

        // Remove listing assuming all went well
        self.unlist_item(Rc::downgrade(&listing));

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::simulation::accounting::bank::Bank;
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
        let owner_a = Actor::new("A", bank.clone(), None);
        let owner_b = Actor::new("B", bank.clone(), None);
        let mut market = Market::new();
        market.list_item(
            Some(owner_a.borrow().id),
            Item {
                id: Uuid::new_v4(),
                kind: "ABC".to_string(),
            },
            500,
        );
        market.list_item(
            Some(owner_b.borrow().id),
            Item {
                id: Uuid::new_v4(),
                kind: "ABC".to_string(),
            },
            750,
        );
        market.list_item(
            Some(owner_a.borrow().id),
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

    #[test]
    fn buy_listing() {
        let bank = Bank::new("Bank");
        let owner_a = Actor::new("A", bank.clone(), None);
        let owner_b = Actor::new("B", bank.clone(), None);
        let mut market = Market::new();
        let _listing_a = market.list_item(
            Some(owner_a.borrow().id),
            Item {
                id: Uuid::new_v4(),
                kind: "ABC".to_string(),
            },
            500,
        );
        let _listing_b = market.list_item(
            Some(owner_b.borrow().id),
            Item {
                id: Uuid::new_v4(),
                kind: "ABC".to_string(),
            },
            750,
        );
        let listing_c = market.list_item(
            Some(owner_a.borrow().id),
            Item {
                id: Uuid::new_v4(),
                kind: "DEF".to_string(),
            },
            250,
        );
        assert_eq!(market.get_listings_of_kind("DEF").len(), 1);
        assert_eq!(market.get_listings_of_kind("ABC").len(), 2);
        assert_eq!(
            market
                .listings_by_owner_id
                .get(&owner_a.borrow().id)
                .unwrap()
                .len(),
            2
        );
        assert_eq!(
            market
                .listings_by_owner_id
                .get(&owner_b.borrow().id)
                .unwrap()
                .len(),
            1
        );
        assert_eq!(
            owner_a
                .borrow()
                .get_account()
                .upgrade()
                .unwrap()
                .borrow()
                .get_balance(),
            0
        );
        assert_eq!(
            owner_b
                .borrow()
                .get_account()
                .upgrade()
                .unwrap()
                .borrow()
                .get_balance(),
            0
        );
        market.buy_listing(listing_c, owner_b.borrow().get_account());
        assert_eq!(market.get_listings_of_kind("DEF").len(), 0);
        assert_eq!(
            market
                .listings_by_owner_id
                .get(&owner_a.borrow().id)
                .unwrap()
                .len(),
            1
        );
        assert_eq!(
            market
                .listings_by_owner_id
                .get(&owner_b.borrow().id)
                .unwrap()
                .len(),
            1
        );
        assert_eq!(
            owner_a
                .borrow()
                .get_account()
                .upgrade()
                .unwrap()
                .borrow()
                .get_balance(),
            250
        );
        assert_eq!(
            owner_b
                .borrow()
                .get_account()
                .upgrade()
                .unwrap()
                .borrow()
                .get_balance(),
            -250
        );
    }
}
