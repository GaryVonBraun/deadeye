use crate::ai::{components::{AiIntent, Blackboard}, tree::{BtNode, BtStatus}};



pub struct IdleAction;

impl BtNode for IdleAction {
    fn tick(&mut self, blackboard: &mut Blackboard) -> BtStatus {
        blackboard.intent = AiIntent::Idle;
        BtStatus::Success
    }
}

pub struct ChaseTarget;

impl BtNode for ChaseTarget {
    fn tick(&mut self, blackboard: &mut Blackboard) -> BtStatus {
        if let Some(target) = blackboard.current_target {
            blackboard.intent = AiIntent::Chase(target);
            BtStatus::Running
        } else {
            BtStatus::Failure
        }
    }
}
