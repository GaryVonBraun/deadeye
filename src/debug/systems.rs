use bevy::prelude::*;

use crate::{
    actor::components::Actor,
    ai::{components::AiController, vision::components::Vision},
    debug::components::DebugMovementIntent,
};

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

pub fn debug_vision_gizmo(query: Query<(&Transform, &Vision)>, mut gizmos: Gizmos) {
    for (transform, vision) in query.iter() {
        gizmos.circle_2d(
            transform.translation.truncate(),
            vision.range,
            Color::srgba(1.0, 1.0, 0.0, 0.5),
        );
    }
}

pub fn debug_visible_entities_gizmo(
    ai_query: Query<(&Transform, &AiController)>,
    actor_query: Query<&Transform, With<Actor>>,
    mut gizmos: Gizmos,
) {
    for (ai_transform, ai_controller) in ai_query.iter() {
        for visible_entity in ai_controller.black_board.visible_actors.iter() {
            let Ok(actor_transform) = actor_query.get(*visible_entity) else {
                continue;
            };

            gizmos.line_2d(
                ai_transform.translation.truncate(),
                actor_transform.translation.truncate(),
                Color::srgba(0.5, 0.5, 0.5, 0.1),
            );
        }
    }
}
pub fn debug_target_entity_gizmo(
    ai_query: Query<(&Transform, &AiController)>,
    actor_query: Query<&Transform, With<Actor>>,
    mut gizmos: Gizmos,
) {
    for (ai_transform, ai_controller) in ai_query.iter() {
        if let Some(target_entity) = ai_controller.black_board.current_target {
            let Ok(actor_transform) = actor_query.get(target_entity) else {
                continue;
            };

            gizmos.arrow_2d(
                ai_transform.translation.truncate(),
                actor_transform.translation.truncate(),
                Color::srgba(1., 0., 0., 1.),
            ).with_tip_length(10.);
        }
    }
}
