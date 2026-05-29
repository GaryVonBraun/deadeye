use bevy::prelude::*;

use crate::navigation::components::NavigationTargetTile;

#[derive(Component, Debug)]
pub struct AStarPath {
    pub target: Option<IVec2>,
    pub calculated_target: Option<IVec2>,
    pub path: Vec<IVec2>,
    pub current_index: usize,
}

#[derive(Bundle)]
pub struct AStarBundle {
    astar: AStarPath,
    target_tile: NavigationTargetTile,
}

impl AStarBundle {
    pub fn default() -> Self {
        AStarBundle {
            astar: AStarPath {
                target: Some(IVec2 { x: 15, y: 15 }),
                calculated_target: None,
                //NOTE - maybe make it optional
                path: vec![],
                current_index: 0,
            },
            target_tile: NavigationTargetTile::default(),
        }
    }
}
