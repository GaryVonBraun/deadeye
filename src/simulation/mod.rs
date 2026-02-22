use bevy::prelude::*;

use crate::simulation::collision::CollisionPlugin;

pub mod collision;
pub struct SimulationPlugin;

impl Plugin for SimulationPlugin {
   fn build(&self, app: &mut App) {
       app.add_plugins(CollisionPlugin);
   } 
}