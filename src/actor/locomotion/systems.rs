use bevy::prelude::*;

use crate::{actor::locomotion::components::Locomotion, debug::components::DebugMovementIntent};

pub fn integrate_movement(
    mut query: Query<(&Locomotion, &mut Transform)>,
    time: Res<Time>,
) {
    for (locomotion, mut transform) in &mut query {
        let direction = locomotion.move_direction.normalize_or_zero();

        let movement = direction * locomotion.speed * time.delta_secs();

        transform.translation += movement.extend(0.0);
    }
}

pub fn resolve_movement(
    mut query: Query<(&mut Locomotion, Option<&DebugMovementIntent>)>,
) {
    for (mut locomotion, debug_movement_component) in query.iter_mut() {
        match debug_movement_component {
            Some(debug_movement) => locomotion.move_direction = debug_movement.intent,
            None => {
                // NOTE - if there is no debug movement we don't have it do nothing
                // NOTE - might have to be careful if component gets removed at runtime, could make the keep its intent.
            },
        }
    }
}
