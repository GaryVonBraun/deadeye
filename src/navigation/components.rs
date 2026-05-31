use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct NavigationTargetTile {
    pub value: Option<IVec2>,
}

impl NavigationTargetTile {
    pub fn default() -> Self {
        NavigationTargetTile { value: None }
    }
}
