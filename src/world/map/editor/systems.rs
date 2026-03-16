use bevy::prelude::*;

use crate::{
    core::states::AppState,
    world::map::{
        messages::{EditMapMessage, LoadMapMessage},
        resources::ActiveMap,
        systems::create_new_map,
    },
};

const CAMERA_SPEED: f32 = 100.;

pub fn init_map_editor(
    active_map: Res<ActiveMap>,
    mut message_writer: MessageWriter<LoadMapMessage>,
) {
    message_writer.write(LoadMapMessage { id: active_map.id });
}

pub fn handle_edit_map_message(
    mut next_state: ResMut<NextState<AppState>>,
    mut active_map: ResMut<ActiveMap>,
    mut edit_map_message_reader: MessageReader<EditMapMessage>,
) {
    for message in edit_map_message_reader.read() {
        active_map.id = message.id;
        next_state.set(AppState::Editor);
    }
}

// pub fn handle_create_map_message(
//     mut next_state: ResMut<NextState<AppState>>,
//     mut active_map: ResMut<ActiveMap>,
// ) {
//     let world_map = create_new_map();
//     active_map.id = world_map.id;
//     next_state.set(AppState::Editor);
// }

pub fn editor_camera_controller(
    keys: Res<ButtonInput<KeyCode>>,
    mut camera_query: Query<&mut Transform, With<Camera>>,
    time: Res<Time>,
) {
    let Ok(mut camera_transform) = camera_query.single_mut() else {
        error!("no camera found");
        return;
    };
    let mut direction = Vec2::default();
    let mut speed_multiplier: f32 = 1.;

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

    //TEMPORARY - for now we just keep it simple and can increase speed with shift
    if keys.pressed(KeyCode::ShiftLeft) {
        speed_multiplier = 2.
    }

    let displacement = direction * (CAMERA_SPEED * speed_multiplier) * time.delta_secs();

    camera_transform.translation += displacement.extend(0.0)
}
