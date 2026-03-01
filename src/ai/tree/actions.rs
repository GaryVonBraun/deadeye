use crate::ai::components::{AiIntent, Blackboard, BtNode, BtStatus};

pub struct Selector {
    pub children: Vec<Box<dyn BtNode>>,
}

impl BtNode for Selector {
    fn tick(&mut self, black_board: &mut Blackboard) -> BtStatus {
        for child in self.children.iter_mut() {
            match child.tick(black_board) {
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
    fn tick(&mut self, blackboard: &mut Blackboard) -> BtStatus {
        for child in self.children.iter_mut() {
            match child.tick(blackboard) {
                BtStatus::Success => continue,
                status => return status,
            }
        }
        BtStatus::Success
    }
}

pub struct IdleAction;

impl BtNode for IdleAction {
    fn tick(&mut self, blackboard: &mut Blackboard) -> BtStatus {
        blackboard.intent = AiIntent::Idle;
        BtStatus::Success
    }
}