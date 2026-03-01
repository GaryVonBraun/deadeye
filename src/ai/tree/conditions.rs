use crate::ai::components::{AiIntent, Blackboard, BtNode, BtStatus};

pub struct HasTarget;

impl BtNode for HasTarget {
    fn tick(&mut self, blackboard: &mut Blackboard) -> BtStatus {
        match blackboard.current_target {
            Some(_) => BtStatus::Success,
            None => BtStatus::Failure,
        }
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
