use std::{
    collections::HashMap,
    rc::{Rc, Weak},
};

use self::{item::Item, listing::Listing};

pub mod item;
pub mod listing;

pub struct Market {
    listings_by_item_kind: HashMap<String, Vec<Rc<Listing>>>,
}
impl Market {
    pub(super) fn new() -> Self {
        Self {
            listings_by_item_kind: HashMap::new(),
        }
    }
    ///
    /// List item on market at a given price. Items that are listed are able to be unlisted.
    ///
    pub(super) fn list_item(&mut self, item: Item, price: i64) {
        if !self.listings_by_item_kind.contains_key(&item.kind) {
            self.listings_by_item_kind
                .insert(item.kind.clone(), Vec::new());
        }
        let listings_of_item_kind = self.listings_by_item_kind.get_mut(&item.kind).unwrap();
        let listing = Rc::new(Listing::new(item, price));
        listings_of_item_kind.push(listing);
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
        if !self.listings_by_item_kind.contains_key(&listing.item.kind) {
            return;
        }
        let listings_of_item_kind = self
            .listings_by_item_kind
            .get_mut(&listing.item.kind)
            .unwrap();
        listings_of_item_kind.retain(|l| l.id != listing.id);
    }
    ///
    /// Get a Vec of Weak references to listings for items of the given kind. There are no
    /// guarentees the Weak references stay valid at any point after the call to this method.
    ///
    pub(super) fn get_listings_of_kind(&self, kind: &str) -> Vec<Weak<Listing>> {
        match self.listings_by_item_kind.get(kind) {
            None => Vec::new(),
            Some(listings) => listings.iter().map(|l| Rc::downgrade(&l)).collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    #[test]
    fn list_item() {
        let mut market = Market::new();
        market.list_item(
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
                .price,
            500
        );
    }

    #[test]
    fn get_listings_of_kind() {
        let mut market = Market::new();
        market.list_item(
            Item {
                id: Uuid::new_v4(),
                kind: "ABC".to_string(),
            },
            500,
        );
        market.list_item(
            Item {
                id: Uuid::new_v4(),
                kind: "ABC".to_string(),
            },
            750,
        );
        market.list_item(
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
    fn unlist_item() {
        let mut market = Market::new();
        market.list_item(
            Item {
                id: Uuid::new_v4(),
                kind: "ABC".to_string(),
            },
            500,
        );
        market.list_item(
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
