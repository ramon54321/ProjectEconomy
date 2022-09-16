use super::item::Item;
use crate::simulation::actor::Actor;
use std::{cell::RefCell, rc::Weak};
use uuid::Uuid;

pub struct Listing {
    pub id: Uuid,
    pub owner: Option<Weak<RefCell<Actor>>>,
    pub item: Item,
    pub price: i64,
}
impl Listing {
    pub(super) fn new(owner: Option<Weak<RefCell<Actor>>>, item: Item, price: i64) -> Self {
        Self {
            id: Uuid::new_v4(),
            owner,
            item,
            price,
        }
    }
}
