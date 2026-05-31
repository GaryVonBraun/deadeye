use crate::ai::{
    components::{AiActionIntent, AiIntent, AiLocomotionIntent, Blackboard},
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

pub struct LocomotionChase;

impl BtNode for LocomotionChase {
    fn tick(&mut self, blackboard: &Blackboard, intent: &mut AiIntent) -> BtStatus {
        if let Some(target) = blackboard.current_target {
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
        if let Some(target) = blackboard.current_target {
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
        if let Some(target) = blackboard.current_target {
            intent.action = AiActionIntent::Melee(target);
            BtStatus::Running
        } else {
            BtStatus::Failure
        }
    }
}
