use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
};
use bevy_egui::{
    EguiContexts,
    egui::{self},
};

use crate::{
    actor::components::Actor,
    ai::{components::AiController, vision::components::Vision},
    collision::{
        self,
        components::{Collision, CollisionShape2d},
    },
    combat::health::components::{Hitbox, Hurtbox},
    debug::{components::DebugMovementIntent, resources::DebugOptions},
    map::{resources::ActiveMap, utility::grid_to_world},
    navigation::astar::components::AStarPath,
};
pub fn display_debug_menu(
    mut contexts: EguiContexts,
    mut debug_options: ResMut<DebugOptions>,
    diagnostics: Res<DiagnosticsStore>,
) -> Result {
    let fps = diagnostics
        .get(&FrameTimeDiagnosticsPlugin::FPS)
        .and_then(|d| d.smoothed());

    egui::Window::new("Debug Menu")
        .resizable(false)
        .show(contexts.ctx_mut()?, |ui| {
            match fps {
                Some(fps) => ui.label(format!("FPS: {fps:.1}")),
                None => ui.label("FPS: --"),
            };

            ui.label("Items");
            ui.toggle_value(&mut debug_options.vision, "Actor vision");
            ui.toggle_value(&mut debug_options.visible_actors, "Visible actors");
            ui.toggle_value(
                &mut debug_options.nearest_visible_hostile,
                "Nearest visible hostile",
            );
            ui.toggle_value(&mut debug_options.hit_box, "Hitbox");
            ui.toggle_value(&mut debug_options.hurt_box, "Hurtbox");
            ui.toggle_value(&mut debug_options.collision, "Collision");
            ui.toggle_value(&mut debug_options.astar_paths, "A* paths");
        });

    Ok(())
}

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

pub fn debug_hitbox_gizmo(query: Query<(&Transform, &Hitbox)>, mut gizmos: Gizmos) {
    for (transform, hitbox) in query.iter() {
        let color = Color::srgba(1.0, 0., 0.0, 1.);

        collision_shape_gizmo(hitbox, transform, color, &mut gizmos);
    }
}

pub fn debug_hurtbox_gizmo(query: Query<(&Transform, &Hurtbox)>, mut gizmos: Gizmos) {
    for (transform, hurtbox) in query.iter() {
        let color = Color::srgba(1.0, 0.5, 0.0, 1.);

        collision_shape_gizmo(hurtbox, transform, color, &mut gizmos);
    }
}

pub fn debug_collision_gizmo(query: Query<(&Transform, &Collision)>, mut gizmos: Gizmos) {
    for (transform, collision) in query.iter() {
        let color = Color::srgba(0., 1., 1., 1.);

        collision_shape_gizmo(collision, transform, color, &mut gizmos);
    }
}

fn collision_shape_gizmo<A: CollisionShape2d>(
    collision: &A,
    transform: &Transform,
    color: Color,
    gizmos: &mut Gizmos,
) {
    let offset_position = transform.translation.truncate() + collision.offset();

    match collision.shape() {
        collision::components::CollisionShape::Circle { radius } => {
            gizmos.circle_2d(offset_position, *radius, color);
        }
        collision::components::CollisionShape::Rect { width, height } => {
            gizmos.rect_2d(
                offset_position,
                Vec2 {
                    x: *width,
                    y: *height,
                },
                color,
            );
        }
    }
}

pub fn debug_visible_entities_gizmo(
    ai_query: Query<(&Transform, &AiController), With<Vision>>,
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
                Color::srgba(0.5, 0.5, 0.5, 0.8),
            );
        }
    }
}
pub fn debug_nearest_visible_hostile_gizmo(
    ai_query: Query<(&Transform, &AiController)>,
    actor_query: Query<&Transform, With<Actor>>,
    mut gizmos: Gizmos,
) {
    for (ai_transform, ai_controller) in ai_query.iter() {
        if let Some(target_entity) = ai_controller.black_board.nearest_visible_hostile {
            let Ok(actor_transform) = actor_query.get(target_entity) else {
                continue;
            };

            gizmos
                .arrow_2d(
                    ai_transform.translation.truncate(),
                    actor_transform.translation.truncate(),
                    Color::srgba(1., 0., 0., 0.1),
                )
                .with_tip_length(10.);
        }
    }
}

pub fn astar_gizmos(
    astar_query: Query<&AStarPath>,
    active_map: Res<ActiveMap>,
    mut gizmos: Gizmos,
) {
    for astar in astar_query.iter() {
        if astar.path.is_empty() || astar.target.is_none() {
            continue;
        }

        let gizmo_color = Color::linear_rgb(1., 0., 0.);

        let path: Vec<Vec2> = astar
            .path
            .iter()
            .skip(astar.current_index)
            .map(|v: &IVec2| {
                grid_to_world(
                    v.x,
                    v.y,
                    active_map.tileset.tile_size,
                    &active_map.map.bounds,
                )
            })
            .collect();

        gizmos.linestrip_2d(path, gizmo_color);
    }
}
