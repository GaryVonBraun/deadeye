use bevy::log::{info, warn};

use crate::ai::{
    components::{AiActionIntent, AiIntent, AiLocomotionIntent, Blackboard},
    directive::components::AiDirective,
    tree::{BtNode, BtStatus},
};

// Locomotion
pub struct LocomotionIdle;

impl BtNode for LocomotionIdle {
    fn tick(&mut self, blackboard: &Blackboard, intent: &mut AiIntent) -> BtStatus {
        intent.locomotion = AiLocomotionIntent::Idle;
        BtStatus::Success
    }
}

pub struct LocomotionFollow;

impl BtNode for LocomotionFollow {
    fn tick(&mut self, blackboard: &Blackboard, intent: &mut AiIntent) -> BtStatus {
        match blackboard.directive {
            AiDirective::Follow(entity) => {
                intent.locomotion = AiLocomotionIntent::Follow(entity);
                BtStatus::Success
            }
            _ => {
                warn!("Ai directive expected to be Follow");
                BtStatus::Failure
            }
        }
    }
}

pub struct LocomotionChaseNearestHostile;

impl BtNode for LocomotionChaseNearestHostile {
    fn tick(&mut self, blackboard: &Blackboard, intent: &mut AiIntent) -> BtStatus {
        if let Some(target) = blackboard.nearest_hostile {
            intent.locomotion = AiLocomotionIntent::Chase(target);
            BtStatus::Running
        } else {
            BtStatus::Failure
        }
    }
}

// Action
pub struct ActionIdle;

impl BtNode for ActionIdle {
    fn tick(&mut self, blackboard: &Blackboard, intent: &mut AiIntent) -> BtStatus {
        intent.action = AiActionIntent::Idle;
        BtStatus::Success
    }
}
pub struct ActionShoot;

impl BtNode for ActionShoot {
    fn tick(&mut self, blackboard: &Blackboard, intent: &mut AiIntent) -> BtStatus {
        if let Some(target) = blackboard.nearest_visible_hostile {
            intent.action = AiActionIntent::Shoot(target);
            BtStatus::Running
        } else {
            BtStatus::Failure
        }
    }
}

pub struct ActionMelee;

impl BtNode for ActionMelee {
    fn tick(&mut self, blackboard: &Blackboard, intent: &mut AiIntent) -> BtStatus {
        //FIXME - currently since zombies are the only ones that melee, its based on the nearest hostile
        //NOTE - ideally it would be better to only call this action of the entity is in range (currently handled by thew melee system)
        if let Some(target) = blackboard.nearest_hostile {
            intent.action = AiActionIntent::Melee(target);
            BtStatus::Running
        } else {
            BtStatus::Failure
        }
    }
}
