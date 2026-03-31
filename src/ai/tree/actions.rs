use bevy::log::info;

use crate::ai::{
    components::{AiActionIntent, AiLocomotionIntent, Blackboard},
    tree::{BtNode, BtStatus},
};

// Locomotion
pub struct LocomotionIdle;

impl BtNode for LocomotionIdle {
    fn tick(&mut self, blackboard: &mut Blackboard) -> BtStatus {
        blackboard.locomotion_intent = AiLocomotionIntent::Idle;
        BtStatus::Success
    }
}

pub struct LocomotionChase;

impl BtNode for LocomotionChase {
    fn tick(&mut self, blackboard: &mut Blackboard) -> BtStatus {
        if let Some(target) = blackboard.current_target {
            blackboard.locomotion_intent = AiLocomotionIntent::Chase(target);
            BtStatus::Running
        } else {
            BtStatus::Failure
        }
    }
}

// Action
pub struct ActionIdle;

impl BtNode for ActionIdle {
    fn tick(&mut self, blackboard: &mut Blackboard) -> BtStatus {
        blackboard.action_intent = AiActionIntent::Idle;
        BtStatus::Success
    }
}
pub struct ActionShoot;

impl BtNode for ActionShoot {
    fn tick(&mut self, blackboard: &mut Blackboard) -> BtStatus {
        if let Some(target) = blackboard.current_target {
            blackboard.action_intent = AiActionIntent::Shoot(target);
            BtStatus::Running
        } else {
            BtStatus::Failure
        }
    }
}

pub struct ActionMelee;

impl BtNode for ActionMelee {
    fn tick(&mut self, blackboard: &mut Blackboard) -> BtStatus {
        if let Some(target) = blackboard.current_target {
            blackboard.action_intent = AiActionIntent::Melee(target);
            BtStatus::Running
        } else {
            BtStatus::Failure
        }
    }
}
