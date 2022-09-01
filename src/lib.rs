use nalgebra_glm::Vec2;

pub mod graphics;
pub mod simulation;

pub struct RenderableState {
    pub banks: Vec<Vec2>,
}
