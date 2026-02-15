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

    let mut direction = Vec2::default();

    if keys.pressed(KeyCode::KeyA) {
        direction.x += -1.;
    }
    if keys.pressed(KeyCode::KeyD) {
        direction.x += 1.;
    }
    if keys.pressed(KeyCode::KeyW) {
        direction.y += 1.;
    }
    if keys.pressed(KeyCode::KeyS) {
        direction.y += -1.;
    }

    movement_entity.direction = direction;
}
