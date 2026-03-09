use bevy::prelude::*;

use crate::ai::{components::*, vision::components::Vision};

#[derive(Bundle)]
pub struct AiBundle {
    controller: AiController,
    locomotion_intent: AiLocomotionIntent,
    action_intent: AiActionIntent,
    vision: Vision,
    movement_intent: AiMovementIntent,
}

impl AiBundle {
    pub fn with_range(vision_range: f32) -> Self {
        AiBundle {
            controller: AiController::default(),
            locomotion_intent: AiLocomotionIntent::default(),
            action_intent: AiActionIntent::default(),
            vision: Vision {
                range: vision_range,
            },
            movement_intent: AiMovementIntent {
                move_direction: Vec2::default(),
                speed: 50.,
            },
        }
    }
}
