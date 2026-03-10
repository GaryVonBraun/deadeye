use bevy::prelude::*;

use crate::{core::states::AppState, world::map::systems::spawn_tilemap};

mod components;
mod systems;
pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), spawn_tilemap);
    }
}
