use uuid::Uuid;

#[derive(Clone)]
pub struct Item {
    pub(super) id: Uuid,
    pub(super) kind: String,
}
