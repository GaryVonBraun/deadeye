use bevy::{ecs::query, prelude::*};

use crate::{actor::locomotion::components::Locomotion, debug::components::DebugMovement};

pub fn debug_movement_controller(
    mut movement_debug_entity: Query<&mut Locomotion, With<DebugMovement>>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    let Ok(mut movement_entity) = movement_debug_entity.single_mut() else {
        return;
    };

    let mut intent = Vec2::default();

    if keys.pressed(KeyCode::KeyA) {
        intent.x += -1.;
    }
    if keys.pressed(KeyCode::KeyD) {
        intent.x += 1.;
    }
    if keys.pressed(KeyCode::KeyW) {
        intent.y += 1.;
    }
    if keys.pressed(KeyCode::KeyS) {
        intent.y += -1.;
    }

    movement_entity.movement_intent = intent;
}
