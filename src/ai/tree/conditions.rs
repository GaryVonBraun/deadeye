use bevy::log::info;

use crate::ai::{
    components::{AiIntent, Blackboard},
    directive::components::AiDirective,
    tree::{BtNode, BtStatus},
};

pub struct HasNearestHostile;

impl BtNode for HasNearestHostile {
    fn tick(&mut self, blackboard: &Blackboard, intent: &mut AiIntent) -> BtStatus {
        match blackboard.nearest_hostile {
            Some(_) => BtStatus::Success,
            None => BtStatus::Failure,
        }
    }
}

pub struct HasNearestVisibleHostile;

impl BtNode for HasNearestVisibleHostile {
    fn tick(&mut self, blackboard: &Blackboard, intent: &mut AiIntent) -> BtStatus {
        match blackboard.nearest_visible_hostile {
            Some(_) => BtStatus::Success,
            None => BtStatus::Failure,
        }
    }
}

pub struct HasFollowDirective;

impl BtNode for HasFollowDirective {
    fn tick(&mut self, blackboard: &Blackboard, intent: &mut AiIntent) -> BtStatus {
        if matches!(blackboard.directive, AiDirective::Follow { .. }) {
            BtStatus::Success
        } else {
            BtStatus::Failure
        }
    }
}
