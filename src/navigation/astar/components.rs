use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct AStarPath {
    pub target: Option<IVec2>,
    pub calculated_target: Option<IVec2>,
    pub path: Vec<IVec2>,
    pub current_index: usize,
}
