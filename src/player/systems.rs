use bevy::{ecs::relationship::Relationship, prelude::*, transform};

use crate::{
    combat::{messages::ShootMessage, weapon::component::Weapon},
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

pub fn player_aim_system(
    window: Single<&mut Window>,
    mut player_query: Query<(&mut PlayerShootingIntent, &GlobalTransform)>,
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

pub fn player_shoot_input(
    mut player_query: Query<(Entity, &mut PlayerShootingIntent)>,
    mut messages: MessageWriter<ShootMessage>,
    buttons: Res<ButtonInput<MouseButton>>,
) {
    let Ok((entity, shooting_intent)) = player_query.single_mut() else {
        return;
    };

    if buttons.just_pressed(MouseButton::Left) {
        //LINK - src/combat/weapon/systems.rs:8
        // this links to where the message is being read

        messages.write(ShootMessage {
            shooter: entity,
            direction: shooting_intent.direction,
        });
    }
}

pub fn rotate_weapons(parent_query: Query<&PlayerShootingIntent>, mut weapon_query: Query<(&ChildOf, &mut Transform), With<Weapon>>) {
    for (parent, mut transform) in weapon_query.iter_mut() {
        if let Ok(intent) = parent_query.get(parent.get()) {
            let angle = intent.direction.to_angle();
            transform.rotation = Quat::from_rotation_z(angle);
        }
    }
}
