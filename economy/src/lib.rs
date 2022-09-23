#![feature(is_some_with)]

use simulation::store::Store;
use uuid::Uuid;
pub mod graphics;
pub mod simulation;

#[derive(Default)]
pub struct RenderableState {
    pub actor_count: usize,
    pub listed_item_kinds: Vec<(String, usize)>,
    pub actor_info: Vec<(Uuid, String, Vec<String>, Store)>,
}
