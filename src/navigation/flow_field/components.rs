use bevy::prelude::*;

use crate::navigation::components::NavigationTargetTile;
#[derive(Component, Debug)]
pub struct FlowFieldTarget {
    pub costs: Vec<Vec<Option<u32>>>,
    pub waypoint_grid: Vec<Vec<Option<IVec2>>>,
    pub last_calculated_tile: Option<IVec2>,
}

impl FlowFieldTarget {
    pub fn default() -> Self {
        FlowFieldTarget {
            costs: vec![],
            waypoint_grid: vec![],
            last_calculated_tile: None,
        }
    }
}

#[derive(Component, Debug)]
pub struct FlowFieldNavigator;

#[derive(Bundle)]
pub struct FlowFieldBundle {
    flow_field: FlowFieldNavigator,
    target_tile: NavigationTargetTile,
}

impl FlowFieldBundle {
    pub fn default() -> Self {
        FlowFieldBundle {
            flow_field: FlowFieldNavigator,
            target_tile: NavigationTargetTile::default(),
        }
    }
}
