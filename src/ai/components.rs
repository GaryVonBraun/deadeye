use bevy::prelude::*;

use crate::ai::tree::{
    BtNode, Selector, Sequence,
    actions::{ActionIdle, LocomotionChase, LocomotionIdle, ActionShoot},
    conditions::HasTarget,
};

#[derive(Component, Default, Debug)]
pub enum AiLocomotionIntent {
    #[default]
    Idle,
    Chase(Entity),
}

#[derive(Component, Default, Debug)]
pub enum AiActionIntent {
    #[default]
    Idle,
    Shoot(Entity),
}

#[derive(Component)]
pub struct Blackboard {
    pub visible_actors: Vec<Entity>,
    pub current_target: Option<Entity>,
    pub locomotion_intent: AiLocomotionIntent,
    pub action_intent: AiActionIntent,
}

#[derive(Component)]
pub struct AiController {
    pub black_board: Blackboard,
    pub action_tree: Box<dyn BtNode>,
    pub locomotion_tree: Box<dyn BtNode>,
}

impl AiController {
    pub fn default() -> Self {
        let action_tree = Box::new(Selector {
            children: vec![
                Box::new(Sequence {
                    children: vec![Box::new(HasTarget), Box::new(ActionShoot)],
                }),
                Box::new(ActionIdle),
            ],
        });
        let locomotion_tree = Box::new(Selector {
            children: vec![
                Box::new(Sequence {
                    children: vec![Box::new(HasTarget), Box::new(LocomotionChase)],
                }),
                Box::new(LocomotionIdle),
            ],
        });

        AiController {
            black_board: Blackboard {
                visible_actors: [].to_vec(),
                current_target: None,
                locomotion_intent: AiLocomotionIntent::Idle,
                action_intent: AiActionIntent::Idle,
            },
            action_tree,
            locomotion_tree,
        }
    }
}

#[derive(Component, Debug)]
pub struct AiMovementIntent {
    pub move_direction: Vec2,
    pub speed: f32,
}
