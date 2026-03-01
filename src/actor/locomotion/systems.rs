use bevy::prelude::*;

use crate::{
    actor::locomotion::components::{AiMovementIntent, Locomotion}, debug::components::DebugMovementIntent,
    player::components::PlayerMovementIntent,
};

pub fn integrate_movement(mut query: Query<(&Locomotion, &mut Transform)>, time: Res<Time>) {
    for (locomotion, mut transform) in &mut query {
        // We normalize our vec2 to get a direction
        let direction = locomotion.move_direction.normalize_or_zero();

        // Displacement is the new position base on the direction x speed and the delta time
        let displacement = direction * locomotion.speed * time.delta_secs();

        transform.translation += displacement.extend(0.0);
    }
}

pub fn resolve_movement(        
    mut query: Query<(
        &mut Locomotion,
        Option<&DebugMovementIntent>,
        Option<&PlayerMovementIntent>,
        Option<&AiMovementIntent>,
    )>,
) {
    for (mut locomotion, debug_intent, player_intent, ai_intent) in query.iter_mut() {
        // When debug_intent is present we map it to the move direction
        if let Some(ai_movement) = ai_intent {
            locomotion.move_direction = ai_movement.move_direction;
            continue;
        }

        locomotion.move_direction = debug_intent.map(|d| d.direction).unwrap_or(Vec2::ZERO);
        locomotion.move_direction = player_intent.map(|d| d.direction).unwrap_or(Vec2::ZERO);
    }
}
