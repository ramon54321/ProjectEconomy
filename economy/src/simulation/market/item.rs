use uuid::Uuid;

#[derive(Clone)]
pub struct Item {
    pub(crate) id: Uuid,
    pub(crate) kind: String,
}
