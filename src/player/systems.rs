use bevy::prelude::*;

use crate::player::components::{PlayerMovementIntent, PlayerShootingIntent};

pub fn player_movement_controller(
    mut query: Query<&mut PlayerMovementIntent>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    let Ok(mut player_entity) = query.single_mut() else {
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

    player_entity.direction = direction;
}

pub fn player_shooting_controller(
    window: Single<&mut Window>,
    mut query: Query<&mut PlayerShootingIntent>,
) {
    let Ok(mut shooting_intent) = query.single_mut() else {
        return;
    };

    let some_position = window.cursor_position();

    match some_position {
        Some(position) => shooting_intent.direction = position.normalize(),
        None => shooting_intent.direction = Vec2::default(),
    }
}
