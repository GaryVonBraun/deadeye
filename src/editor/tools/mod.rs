use bevy::prelude::*;
use bevy_egui::EguiContexts;

use crate::{
    editor::{
        component::PlacementPreview,
        resources::{EditorSettings, EditorTool, ToolAction},
        tools::{
            prop_tool::{prop_tile_aligned, prop_tool_system},
            tile_painter::tile_paint_system,
        },
    },
    map::resources::ActiveMap,
    mission::resources::ActiveMission,
    props::{
        components::Prop,
        io::operations::read_prop_definitions,
        messages::{RemovePropMessage, SpawnPropMessage},
        resources::ActiveMapProps,
    },
};

mod prop_tool;
mod tile_painter;

pub fn editor_click_system(
    mouse: Res<ButtonInput<MouseButton>>,
    window: Single<&Window>,
    camera_query: Single<(&Camera, &GlobalTransform)>,
    editor_tool: Res<EditorTool>,
    active_map: ResMut<ActiveMap>,
    mut active_mission: ResMut<ActiveMission>,
    mut contexts: EguiContexts,
    editor_settings: Res<EditorSettings>,

    // props
    place_prop_writer: MessageWriter<SpawnPropMessage>,
    remove_prop_writer: MessageWriter<RemovePropMessage>,
    prop_query: Query<&PlacementPreview>,
    active_map_props: ResMut<ActiveMapProps>,
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

    let tile_position = Vec2 {
        x: ((world_pos.x + active_map.map.bounds.west as f32 * tile_size) / tile_size).floor(),
        y: ((active_map.map.bounds.north as f32 * tile_size - world_pos.y) / tile_size).floor(),
    };

    let tile_world_position = Vec2 {
        x: tile_world_position(world_pos.x, active_map.tileset.tile_size),
        y: tile_world_position(world_pos.y, active_map.tileset.tile_size),
    };

    // bounds check
    if tile_position.x < 0.
        || tile_position.y < 0.
        || tile_position.x >= map_width as f32
        || tile_position.y >= map_height as f32
    {
        return;
    }

    match &*editor_tool {
        EditorTool::TilePainter(tile_index) => {
            tile_paint_system(tile_index, active_map, tile_position);
        }
        EditorTool::PlayerSpawn => {
            info!("setting player spawnpoint");

            active_mission.mission.player_spawn = tile_world_position;
        }
        EditorTool::PropTool(action) => {
            if !mouse.just_pressed(MouseButton::Left) {
                return;
            }

            let Ok(preview) = prop_query.single() else {
                return;
            };

            prop_tool_system(
                action,
                world_pos,
                editor_settings,
                preview,
                active_map,
                active_map_props,
                place_prop_writer,
                remove_prop_writer,
            );
        }
        EditorTool::None => {}
    };
}

fn tile_world_position(position: f32, tile_size: f32) -> f32 {
    let floored = (position / tile_size).floor();
    floored * tile_size + tile_size
}

pub fn update_preview_image(
    mut query: Query<(&mut Visibility, &mut Sprite, &mut PlacementPreview)>,
    editor_tool: Res<EditorTool>,
    asset_server: Res<AssetServer>,
    images: Res<Assets<Image>>,
) {
    let Ok((mut visibility, mut sprite, mut preview)) = query.single_mut() else {
        warn!("Trying to update placement image but None found");
        return;
    };

    match &*editor_tool {
        EditorTool::None => {}
        EditorTool::TilePainter(_) => {
            *visibility = Visibility::Hidden;
        }
        EditorTool::PropTool(tool_action) => match tool_action {
            ToolAction::Place(name) => {
                let Ok(definitions) = read_prop_definitions() else {
                    return;
                };

                *visibility = Visibility::Visible;

                let Some(prop_definition) = definitions
                    .props
                    .iter()
                    .find(|definition| definition.name == name.to_string())
                else {
                    error!("Failed to find definition for placed prop");
                    //TODO - perhaps in the future we can spawn the prop but show a missing texture
                    return;
                };

                sprite.image = asset_server.load(format!("props/{}.png", prop_definition.sprite));
                sprite.color = Color::srgba(1.0, 1.0, 1.0, 0.5);

                // this is so we know how to offset the preview
                preview.size = prop_definition.size;
            }
            ToolAction::Remove => *visibility = Visibility::Hidden,
        },
        EditorTool::PlayerSpawn => *visibility = Visibility::Hidden,
    }
}

pub fn update_preview_position(
    window: Single<&Window>,
    camera_query: Single<(&Camera, &GlobalTransform)>,
    mut query: Query<(&PlacementPreview, &mut Transform)>,
    active_map: ResMut<ActiveMap>,
    editor_settings: Res<EditorSettings>,
) {
    let Ok((preview, mut transform)) = query.single_mut() else {
        warn!("Trying to draw placement position but None found");
        return;
    };

    let Some(cursor_pos) = window.cursor_position() else {
        return;
    };

    let Ok(world_pos) = camera_query
        .0
        .viewport_to_world_2d(camera_query.1, cursor_pos)
    else {
        return;
    };

    if editor_settings.snap_to_grid {
        transform.translation =
            prop_tile_aligned(world_pos, active_map.tileset.tile_size, preview.size).extend(0.);
    } else {
        transform.translation = world_pos.extend(0.);
    }
}
