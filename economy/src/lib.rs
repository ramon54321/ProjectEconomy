#![feature(is_some_with)]

pub mod graphics;
pub mod simulation;

#[derive(Default)]
pub struct RenderableState {
    pub actor_count: usize,
    pub listed_item_kinds: Vec<(String, usize)>,
}
