use bevy::prelude::*;
#[derive(Component, Debug)]
pub struct FlowFieldTarget {
    pub costs: Vec<Vec<Option<u32>>>,
    pub directions: Vec<Vec<Option<Vec2>>>,
    pub last_calculated_tile: Option<(i32, i32)>,
}

impl FlowFieldTarget {
    pub fn default() -> Self {
        FlowFieldTarget {
            costs: vec![],
            directions: vec![],
            last_calculated_tile: None,
        }
    }
}
