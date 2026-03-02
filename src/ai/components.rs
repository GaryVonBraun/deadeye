use bevy::prelude::*;

use crate::ai::tree::{
    actions::{IdleAction, Selector, Sequence},
    conditions::{ChaseTarget, HasTarget},
};

pub enum BtStatus {
    Failure,
    Success,
    Running,
}

pub trait BtNode: Send + Sync {
    fn tick(&mut self, black_board: &mut Blackboard) -> BtStatus;
}

#[derive(Component, Default, Debug)]
pub enum AiIntent {
    #[default]
    Idle,
    Chase(Entity),
    Attack(Entity),
}

#[derive(Component)]
pub struct Blackboard {
    pub visible_actors: Vec<Entity>,
    pub current_target: Option<Entity>,
    pub last_known_enemy_pos: Option<Vec2>,
    pub intent: AiIntent,
}

#[derive(Component)]
pub struct AiController {
    pub black_board: Blackboard,
    pub tree: Box<dyn BtNode>,
}

impl AiController {
    pub fn default() -> Self {
        let tree = Box::new(Selector {
            children: vec![
                Box::new(Sequence {
                    children: vec![Box::new(HasTarget), Box::new(ChaseTarget)],
                }),
                Box::new(IdleAction),
            ],
        });

        AiController {
            black_board: Blackboard {
                visible_actors: [].to_vec(),
                current_target: None,
                last_known_enemy_pos: None,
                intent: AiIntent::Idle,
            },
            tree: tree,
        }
    }
}


#[derive(Component, Debug)]
pub struct AiMovementIntent {
    pub move_direction: Vec2,
    pub speed: f32,
}
