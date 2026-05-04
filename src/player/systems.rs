use bevy::{
    input::mouse::{MouseScrollUnit, MouseWheel},
    prelude::*,
};

use crate::{
    combat::{
        components::ShootingIntent,
        messages::{ReloadMessage, ShootMessage},
    },
    player::components::{Player, PlayerMovementIntent},
};

pub fn player_movement_controller(
    mut query: Query<&mut PlayerMovementIntent>,
    keys: Res<ButtonInput<KeyCode>>,
    mut camera_query: Query<&mut Projection, With<Camera>>,
    mut evr_scroll: MessageReader<MouseWheel>,
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

    //FIXME - just a quick implementation so i can test better

    let Ok(mut projection) = camera_query.single_mut() else {
        error!("no camera found");
        return;
    };

    for ev in evr_scroll.read() {
        match ev.unit {
            MouseScrollUnit::Line => {
                if let Projection::Orthographic(ref mut ortho) = *projection {
                    ortho.scale -= ev.y * 0.1;
                    ortho.scale = ortho.scale.clamp(0.1, 50.0);
                }
            }
            MouseScrollUnit::Pixel => {
                // do nothing
            }
        }
    }
}

pub fn player_aim_system(
    window: Single<&mut Window>,
    mut player_query: Query<(&mut ShootingIntent, &GlobalTransform), With<Player>>,
    camera_query: Single<(&Camera, &GlobalTransform)>,
) {
    let Ok((mut shooting_intent, transform)) = player_query.single_mut() else {
        return;
    };

    let Some(cursor_position) = window.cursor_position() else {
        return;
    };

    let Ok(mouse_world_position) = camera_query
        .0
        .viewport_to_world_2d(camera_query.1, cursor_position)
    else {
        return;
    };

    let player_position = transform.translation().truncate();

    shooting_intent.direction = (mouse_world_position
        - Vec2 {
            x: player_position.x,
            y: player_position.y,
        })
    .normalize_or_zero();
}

pub fn player_combat_input(
    mut player_query: Query<(Entity, &mut ShootingIntent), With<Player>>,
    mut shoot_writer: MessageWriter<ShootMessage>,
    mut reload_writer: MessageWriter<ReloadMessage>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    let Ok((entity, shooting_intent)) = player_query.single_mut() else {
        return;
    };

    if mouse_buttons.pressed(MouseButton::Left) {
        //LINK - src/combat/weapon/systems.rs:8
        // this links to where the message is being read

        shoot_writer.write(ShootMessage {
            owner: entity,
            direction: shooting_intent.direction,
        });
    }
    if keys.pressed(KeyCode::KeyR) {
        reload_writer.write(ReloadMessage { entity: entity });
    }
}

pub fn camera_follow_player(
    mut camera_query: Query<&mut Transform, (With<Camera>, Without<Player>)>,
    player_query: Query<&Transform, With<Player>>,
) {
    let Ok(mut camera_transform) = camera_query.single_mut() else {
        error!("no camera found");
        return;
    };

    let Ok(player_transform) = player_query.single() else {
        return;
    };
    camera_transform.translation = player_transform.translation;
}
