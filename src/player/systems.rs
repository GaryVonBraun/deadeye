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
    mut query: Query<(Entity, &mut PlayerShootingIntent)>,
    mut messages: MessageWriter<ShootMessage>,
    buttons: Res<ButtonInput<MouseButton>>,
) {
    
    let Ok((entity, mut shooting_intent)) = query.single_mut() else {
        return;
    };

    let some_position = window.cursor_position();

    match some_position {
        Some(position) => {
            shooting_intent.direction = position.normalize();
            if buttons.just_pressed(MouseButton::Left) {
                info!("clicked mouse button");
                messages.write(ShootMessage {
                    shooter: entity,
                    direction: shooting_intent.direction,
                });
            }
        }
        None => shooting_intent.direction = Vec2::default(),
    }
}
