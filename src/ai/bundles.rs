use bevy::prelude::*;

use crate::{
    ai::{components::*, directive::components::AiDirective, vision::components::Vision},
    navigation::astar::components::AStarPath,
};

#[derive(Bundle)]
pub struct BaseAiBundle {
    controller: AiController,
    locomotion_intent: AiLocomotionIntent,
    action_intent: AiActionIntent,
    movement_intent: AiMovementIntent,
}

impl BaseAiBundle {
    pub fn with_controller(ai_controller: AiController) -> Self {
        BaseAiBundle {
            controller: ai_controller,
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
    directive: AiDirective,
    vision: Vision,
    astar: AStarPath,
}

impl SentientAiBundle {
    pub fn with_vision_range(vision_range: f32) -> Self {
        SentientAiBundle {
            base_ai: BaseAiBundle {
                controller: AiController::default_human(),
                locomotion_intent: AiLocomotionIntent::default(),
                action_intent: AiActionIntent::default(),

                movement_intent: AiMovementIntent {
                    move_direction: Vec2::default(),
                },
            },
            directive: AiDirective::SearchAndDestroy,
            vision: Vision {
                range: vision_range,
            },
            astar: AStarPath {
                target: Some(IVec2 { x: 10, y: 10 }),
                calculated_target: None,
                path: vec![],
                current_index: 0,
            },
        }
    }
}
