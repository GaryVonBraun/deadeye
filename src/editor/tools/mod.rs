use bevy::prelude::*;
use bevy_egui::EguiContexts;

use crate::{
    editor::{
        resources::{ActiveTile, EditorTool},
        tools::tile_painter::tile_paint_system,
    },
    map::resources::ActiveMap,
    mission::resources::ActiveMission,
};

mod tile_painter;

pub fn editor_click_system(
    mouse: Res<ButtonInput<MouseButton>>,
    window: Single<&Window>,
    camera_query: Single<(&Camera, &GlobalTransform)>,
    editor_tool: Res<EditorTool>,
    active_tile: Res<ActiveTile>,
    active_map: ResMut<ActiveMap>,
    mut active_mission: ResMut<ActiveMission>,
    mut contexts: EguiContexts,
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

    match *editor_tool {
        EditorTool::TilePainter => {
            tile_paint_system(active_tile, active_map, tile_position);
        }
        EditorTool::PlayerSpawn => {
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
    };
}

fn position_tile_center(position: f32, tile_size: f32) -> f32 {
    let ceiled_position = (position / tile_size).signum() * (position / tile_size).abs().ceil();
    ceiled_position * 64. + 32.
}
