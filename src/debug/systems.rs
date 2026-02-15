use bevy::prelude::*;

use crate::debug::components::DebugMovementIntent;

//TEMPORARY - This is a quick implementation to see if the locomotion system works
pub fn debug_movement_controller(
    mut movement_debug_entity: Query<&mut DebugMovementIntent>,
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

    movement_entity.intent = intent;
}
