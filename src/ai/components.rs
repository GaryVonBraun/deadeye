use bevy::prelude::*;

#[derive(Component, Default)]
pub enum AiIntent {
    #[default]
    Idle,
    Chase(Entity),
    Attack(Entity),
}

#[derive(Component)]
pub struct BlackBoard {
    pub visible_actors: Vec<Entity>,
    pub current_target: Option<Entity>,
    pub last_known_enemy_pos: Option<Vec2>,
}

#[derive(Component)]
pub struct AiController {
    pub black_board: BlackBoard,
}

impl AiController {
    pub fn default() -> Self {
        AiController {
            black_board: BlackBoard {
                visible_actors: [].to_vec(),
                current_target: None,
                last_known_enemy_pos: None,
            },
        }
    }
}
