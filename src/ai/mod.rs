use bevy::prelude::*;

use crate::{
    ai::{directive::DirectivePlugin, systems::*, tree::BehaviorTreePlugin, vision::VisionPlugin},
    core::states::SimulationState,
    map::resources::ActiveMap,
};

pub mod bundles;
pub mod components;
mod directive;
mod systems;
pub mod tree;
pub mod vision;

pub struct AiPlugin;

impl Plugin for AiPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            Update,
            (AiSet::Perception, AiSet::Targeting, AiSet::Decision).chain(),
        );
        app.add_plugins((VisionPlugin, BehaviorTreePlugin, DirectivePlugin));
        app.add_systems(
            Update,
            (
                // ai_movement_system,
                (
                    target_navigation,
                    follow_target_actor,
                    // separation_steering
                )
                    .run_if(resource_exists::<ActiveMap>),
                ai_shooting_system,
                ai_melee_system,
                seek_nearest_hostile,
            )
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
