use bevy::prelude::*;

use crate::actor::humanoid::appearance::HumanoidAppearancePlugin;

mod appearance;
pub mod factories;
pub struct HumanoidPlugin;

impl Plugin for HumanoidPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(HumanoidAppearancePlugin);
    }
}