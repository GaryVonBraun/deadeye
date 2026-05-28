use crate::ai::{
    components::Blackboard,
    tree::{BtNode, BtStatus},
};

pub struct HasTarget;

impl BtNode for HasTarget {
    fn tick(&mut self, blackboard: &mut Blackboard) -> BtStatus {
        match blackboard.current_target {
            Some(_) => BtStatus::Success,
            None => BtStatus::Failure,
        }
    }
}

pub struct FollowingTarget;

impl BtNode for FollowingTarget {
    fn tick(&mut self, blackboard: &mut Blackboard) -> BtStatus {
        match blackboard.current_target {
            Some(_) => BtStatus::Success,
            None => BtStatus::Failure,
        }
    }
}
