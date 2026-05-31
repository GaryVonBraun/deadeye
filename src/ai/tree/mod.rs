use bevy::prelude::*;

use crate::{
    ai::{
        AiSet,
        components::{AiIntent, Blackboard},
        tree::systems::behavior_tree_system,
    },
    core::states::SimulationState,
};

pub mod actions;
pub mod conditions;
mod systems;
pub struct BehaviorTreePlugin;

impl Plugin for BehaviorTreePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            behavior_tree_system
                .in_set(AiSet::Decision)
                .run_if(in_state(SimulationState::Running)),
        );
    }
}

pub enum BtStatus {
    Failure,
    Success,
    Running,
}

pub trait BtNode: Send + Sync {
    fn tick(&mut self, black_board: &Blackboard, intent: &mut AiIntent) -> BtStatus;
}

pub struct Selector {
    pub children: Vec<Box<dyn BtNode>>,
}

impl BtNode for Selector {
    fn tick(&mut self, black_board: &Blackboard, intent: &mut AiIntent) -> BtStatus {
        for child in self.children.iter_mut() {
            match child.tick(black_board, intent) {
                BtStatus::Failure => continue,
                status => return status,
            }
        }
        BtStatus::Failure
    }
}
pub struct Sequence {
    pub children: Vec<Box<dyn BtNode>>,
}

impl BtNode for Sequence {
    fn tick(&mut self, blackboard: &Blackboard, intent: &mut AiIntent) -> BtStatus {
        for child in self.children.iter_mut() {
            match child.tick(blackboard, intent) {
                BtStatus::Success => continue,
                status => return status,
            }
        }
        BtStatus::Success
    }
}
