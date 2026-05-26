use bevy::prelude::*;

use crate::ai::tree::{
    BtNode, Selector, Sequence,
    actions::{ActionIdle, ActionMelee, ActionShoot, LocomotionChase, LocomotionIdle},
    conditions::HasTarget,
};

#[derive(Component, Default, Debug)]
pub enum AiLocomotionIntent {
    #[default]
    Idle,
    Chase(Entity),
}

#[derive(Component, Debug)]
pub struct SeekNearestTarget;

// ai decision making
#[derive(Component, Default, Debug)]
pub enum AiActionIntent {
    #[default]
    Idle,
    Shoot(Entity),
    Melee(Entity),
}

#[derive(Component)]
pub struct Blackboard {
    pub visible_actors: Vec<Entity>,
    pub current_target: Option<Entity>,
    pub locomotion_intent: AiLocomotionIntent,
    pub action_intent: AiActionIntent,
}

#[derive(Component, Debug)]
pub struct AiMovementIntent {
    pub move_direction: Vec2,
}

#[derive(Component)]
pub struct AiController {
    pub black_board: Blackboard,
    pub action_tree: Box<dyn BtNode>,
    pub locomotion_tree: Box<dyn BtNode>,
}

impl AiController {
    pub fn default_human() -> Self {
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
    pub fn zombie() -> Self {
        let action_tree = Box::new(Selector {
            children: vec![
                Box::new(Sequence {
                    children: vec![Box::new(HasTarget), Box::new(ActionMelee)],
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
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum AiSet {
    Perception,
    Targeting,
    Decision,
}
