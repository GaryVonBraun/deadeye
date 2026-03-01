use bevy::prelude::*;

use crate::{
    ai::{systems::*, tree::BehaviorTreePlugin, vision::VisionPlugin},
    core::states::SimulationState,
};

pub mod bundles;
mod components;
mod systems;
pub mod tree;
mod vision;

pub struct AiPlugin;

impl Plugin for AiPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            Update,
            (AiSet::Perception, AiSet::Targeting, AiSet::Decision).chain(),
        );
        app.add_plugins((VisionPlugin, BehaviorTreePlugin));
        app.add_systems(
            Update,
            (ai_targeting_system, ai_movement_system)
                .in_set(AiSet::Targeting)
                .run_if(in_state(SimulationState::Running)),
        );
    }
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum AiSet {
    Perception,
    Targeting,
    Decision,
}
