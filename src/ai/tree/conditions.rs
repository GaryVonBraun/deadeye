use crate::ai::{
    components::{AiIntent, Blackboard},
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

