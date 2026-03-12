use bevy::prelude::*;

use crate::ui::map::menu::MapsMenuUiPlugin;

mod menu;
pub struct MapUiPlugin;

impl Plugin for MapUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MapsMenuUiPlugin);
    }
}
