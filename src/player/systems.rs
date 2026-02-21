use bevy::prelude::*;

use crate::{
    combat::messages::ShootMessage,
    player::components::{PlayerMovementIntent, PlayerShootingIntent},
};

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
    mut player_query: Query<(Entity, &mut PlayerShootingIntent, &GlobalTransform)>,
    camera_query: Single<(&Camera, &GlobalTransform)>,
    mut messages: MessageWriter<ShootMessage>,
    buttons: Res<ButtonInput<MouseButton>>,
) {

    //FIXME - this looks bad

    if !buttons.just_pressed(MouseButton::Left) {
        return;
    }

    let Ok((entity, mut shooting_intent, transform)) = player_query.single_mut() else {
        return;
    };

    let Some(mouse_position) = window.cursor_position() else {
        return;
    };

    let Ok(mouse_world) = camera_query
        .0
        .viewport_to_world_2d(camera_query.1, mouse_position)
    else {
        return;
    };

    let player_position = transform.translation().truncate();

    shooting_intent.direction = (mouse_world
        - Vec2 {
            x: player_position.x,
            y: player_position.y,
        })
    .normalize_or_zero();

    //LINK - src/combat/weapon/systems.rs:8
    // this links to where the message is being read

    messages.write(ShootMessage {
        shooter: entity,
        direction: shooting_intent.direction,
    });
}
