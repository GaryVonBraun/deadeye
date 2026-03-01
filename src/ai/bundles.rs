use bevy::prelude::*;

use crate::ai::{components::*, vision::components::Vision};

#[derive(Bundle)]
pub struct AiBundle {
    controller: AiController,
    intent: AiIntent,
    vision: Vision,
}

impl AiBundle {
    pub fn with_range(vision_range: f32) -> Self {
        AiBundle {
            controller: AiController::default(),
            intent: AiIntent::default(),
            vision: Vision {
                range: vision_range,
            },
        }
    }
}
