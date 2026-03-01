use bevy::prelude::*;

use crate::{ai::{AiSet, tree::systems::behavior_tree_system}, core::states::SimulationState};

pub mod actions;
pub mod conditions;
mod systems;
pub struct BehaviorTreePlugin;

impl Plugin for BehaviorTreePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, behavior_tree_system.in_set(AiSet::Decision).run_if(in_state(SimulationState::Running)));
    }
}