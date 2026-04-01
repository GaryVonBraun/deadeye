use bevy::prelude::*;
use bevy_egui::EguiContexts;

use crate::{
    editor::{
        resources::EditorTools,
        tools::{prop_tool::prop_tool_system, tile_painter::tile_paint_system},
    },
    map::resources::ActiveMap,
    mission::resources::ActiveMission,
    props::messages::SpawnPropMessage,
};

mod prop_tool;
mod tile_painter;

pub fn editor_click_system(
    mouse: Res<ButtonInput<MouseButton>>,
    window: Single<&Window>,
    camera_query: Single<(&Camera, &GlobalTransform)>,
    editor_tool: Res<EditorTools>,
    active_map: ResMut<ActiveMap>,
    mut active_mission: ResMut<ActiveMission>,
    mut contexts: EguiContexts,

    // props
    place_prop_writer: MessageWriter<SpawnPropMessage>,
) {
    let Ok(ctx) = contexts.ctx_mut() else {
        return;
    };
    if ctx.is_pointer_over_area() {
        return;
    }

    if !mouse.pressed(MouseButton::Left) {
        return;
    }

    let Some(cursor_pos) = window.cursor_position() else {
        return;
    };
    let Ok(world_pos) = camera_query
        .0
        .viewport_to_world_2d(camera_query.1, cursor_pos)
    else {
        return;
    };

    let tile_size = active_map.tileset.tile_size;
    let map_width = active_map.map.tiles[0].len();
    let map_height = active_map.map.tiles.len();

    let tile_x =
        ((world_pos.x + active_map.map.bounds.west as f32 * tile_size) / tile_size).floor() as i32;
    let tile_y =
        ((active_map.map.bounds.north as f32 * tile_size - world_pos.y) / tile_size).floor() as i32;

    let tile_position = Vec2 {
        x: tile_x as f32,
        y: tile_y as f32,
    };

    // bounds check
    if tile_x < 0 || tile_y < 0 || tile_x >= map_width as i32 || tile_y >= map_height as i32 {
        return;
    }

    match &*editor_tool {
        EditorTools::TilePainter(tile_index) => {
            tile_paint_system(tile_index, active_map, tile_position);
        }
        EditorTools::PlayerSpawn => {
            info!("setting player spawnpoint");

            info!(
                "{}",
                position_tile_center(world_pos.x, active_map.tileset.tile_size)
            );
            info!(
                "{}",
                position_tile_center(world_pos.y, active_map.tileset.tile_size)
            );
            active_mission.mission.player_spawn = vec2(
                position_tile_center(world_pos.x, active_map.tileset.tile_size),
                position_tile_center(world_pos.y, active_map.tileset.tile_size),
            )
        }
        EditorTools::PropTool(action) => {
            if !mouse.just_pressed(MouseButton::Left) {
                return;
            }
            prop_tool_system(action, world_pos, place_prop_writer)
        }
        EditorTools::None => {}
    };
}

fn position_tile_center(position: f32, tile_size: f32) -> f32 {
    let floored = (position / tile_size).floor();
    floored * tile_size + tile_size / 2.0
}
