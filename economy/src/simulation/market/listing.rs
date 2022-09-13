use super::item::Item;
use uuid::Uuid;

pub struct Listing {
    pub id: Uuid,
    pub item: Item,
    pub price: i64,
}
impl Listing {
    pub(super) fn new(item: Item, price: i64) -> Self {
        Self {
            id: Uuid::new_v4(),
            item,
            price,
        }
    }
}
