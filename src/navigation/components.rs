use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct NavigationTargetTile {
    target: IVec2,
}

impl NavigationTargetTile {
    pub fn default() -> Self {
        NavigationTargetTile {
            target: IVec2::default(),
        }
    }
}
