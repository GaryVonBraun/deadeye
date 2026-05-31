use crate::ai::{
    components::{AiIntent, Blackboard},
    tree::{BtNode, BtStatus},
};

pub struct HasTarget;

impl BtNode for HasTarget {
    fn tick(&mut self, blackboard: &Blackboard, intent: &mut AiIntent) -> BtStatus {
        match blackboard.current_target {
            Some(_) => BtStatus::Success,
            None => BtStatus::Failure,
        }
    }
}

pub struct FollowingTarget;

impl BtNode for FollowingTarget {
    fn tick(&mut self, blackboard: &Blackboard, intent: &mut AiIntent) -> BtStatus {
        match blackboard.current_target {
            Some(_) => BtStatus::Success,
            None => BtStatus::Failure,
        }
    }
}
