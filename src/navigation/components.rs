use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct NavigationTargetTile(pub Option<IVec2>);

impl NavigationTargetTile {
    pub fn default() -> Self {
        NavigationTargetTile(None)
    }
}
