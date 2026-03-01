use bevy::prelude::*;

use crate::{
    actor::locomotion::components::AiMovementIntent,
    ai::{components::*, vision::components::Vision},
};

#[derive(Bundle)]
pub struct AiBundle {
    controller: AiController,
    intent: AiIntent,
    vision: Vision,
    movement_intent: AiMovementIntent,
}

impl AiBundle {
    pub fn with_range(vision_range: f32) -> Self {
        AiBundle {
            controller: AiController::default(),
            intent: AiIntent::default(),
            vision: Vision {
                range: vision_range,
            },
            movement_intent: AiMovementIntent { move_direction: Vec2::default(), speed: 50. }
        }
    }
}
