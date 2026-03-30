use bevy::prelude::*;

use crate::ai::{components::*, vision::components::Vision};

#[derive(Bundle)]
pub struct BaseAiBundle {
    controller: AiController,
    locomotion_intent: AiLocomotionIntent,
    action_intent: AiActionIntent,
    movement_intent: AiMovementIntent,
}

impl BaseAiBundle {
    pub fn default() -> Self {
        BaseAiBundle {
            controller: AiController::default(),
            locomotion_intent: AiLocomotionIntent::default(),
            action_intent: AiActionIntent::default(),

            movement_intent: AiMovementIntent {
                move_direction: Vec2::default(),
            },
        }
    }
}

#[derive(Bundle)]
pub struct SentientAiBundle {
    base_ai: BaseAiBundle,
    vision: Vision,
}

impl SentientAiBundle {
    pub fn with_vision_range(vision_range: f32) -> Self {
        SentientAiBundle {
            base_ai: BaseAiBundle {
                controller: AiController::default(),
                locomotion_intent: AiLocomotionIntent::default(),
                action_intent: AiActionIntent::default(),

                movement_intent: AiMovementIntent {
                    move_direction: Vec2::default(),
                },
            },
            vision: Vision {
                range: vision_range,
            },
        }
    }
}
