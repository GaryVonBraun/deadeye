use bevy::prelude::*;

use crate::ai::{
    directive::components::AiDirective,
    tree::{BtNode, Selector, Sequence, actions::*, conditions::*},
};

#[derive(Component, Default, Debug)]
pub enum AiLocomotionIntent {
    #[default]
    Idle,
    Chase(Entity),
    Follow {
        target: Entity,
        distance: f32,
    },
}

#[derive(Component, Debug)]
pub struct SeekNearestHostile;

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
    pub nearby_actors: Vec<Entity>,
    pub visible_actors: Vec<Entity>,
    pub nearest_visible_hostile: Option<Entity>,
    pub nearest_hostile: Option<Entity>,
    pub directive: AiDirective,
}

impl Blackboard {
    pub fn default() -> Self {
        Blackboard {
            nearby_actors: vec![],
            visible_actors: vec![],
            nearest_visible_hostile: None,
            nearest_hostile: None,
            directive: AiDirective::Idle,
        }
    }
}

#[derive(Component, Debug)]
pub struct AiMovementIntent {
    pub move_direction: Vec2,
}

#[derive(Component, Debug)]
pub struct AiIntent {
    pub locomotion: AiLocomotionIntent,
    pub action: AiActionIntent,
}

impl AiIntent {
    fn idle() -> Self {
        AiIntent {
            locomotion: AiLocomotionIntent::Idle,
            action: AiActionIntent::Idle,
        }
    }
}

#[derive(Component)]
pub struct AiController {
    pub black_board: Blackboard,
    pub action_tree: Box<dyn BtNode>,
    pub locomotion_tree: Box<dyn BtNode>,
    pub intent: AiIntent,
}

impl AiController {
    pub fn default_human() -> Self {
        let action_tree = Box::new(Selector {
            children: vec![
                Box::new(Sequence {
                    children: vec![Box::new(HasNearestVisibleHostile), Box::new(ActionShoot)],
                }),
                Box::new(ActionIdle),
            ],
        });
        let locomotion_tree = Box::new(Selector {
            children: vec![
                Box::new(Sequence {
                    children: vec![Box::new(HasFollowDirective), Box::new(LocomotionFollow)],
                }),
                Box::new(LocomotionIdle),
            ],
        });

        AiController {
            black_board: Blackboard::default(),
            intent: AiIntent::idle(),
            action_tree,
            locomotion_tree,
        }
    }
    pub fn zombie() -> Self {
        let action_tree = Box::new(Selector {
            children: vec![
                Box::new(Sequence {
                    children: vec![Box::new(HasNearestHostile), Box::new(ActionMelee)],
                }),
                Box::new(ActionIdle),
            ],
        });
        let locomotion_tree = Box::new(Selector {
            children: vec![
                Box::new(Sequence {
                    children: vec![
                        Box::new(HasNearestHostile),
                        Box::new(LocomotionChaseNearestHostile),
                    ],
                }),
                Box::new(LocomotionIdle),
            ],
        });

        AiController {
            black_board: Blackboard::default(),
            intent: AiIntent::idle(),
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
