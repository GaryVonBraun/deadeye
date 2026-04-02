use bevy::prelude::*;

use crate::{
    editor::{
        resources::{EditorSettings, ToolAction},
        tools::tile_world_position,
    },
    map::resources::ActiveMap,
    props::{io::types::PlacedProp, messages::SpawnPropMessage},
};

pub fn prop_tool_system(
    action: &ToolAction,
    world_pos: Vec2,
    mut place_prop_writer: MessageWriter<SpawnPropMessage>,
    editor_settings: Res<EditorSettings>,
    prop_sprite: &Sprite,
    mut active_map: ResMut<ActiveMap>,
) {
    match action {
        ToolAction::Place(name) => {
            let placement_position: Vec2;

            if editor_settings.snap_to_grid {
                let Some(size) = prop_sprite.custom_size else {
                    return;
                };
                placement_position =
                    prop_tile_aligned(world_pos, active_map.tileset.tile_size, size);
            } else {
                placement_position = world_pos;
            }

            place_prop_writer.write(SpawnPropMessage {
                name: name.to_string(),
                position: placement_position,
            });
            active_map.map.placed_props.push(PlacedProp {
                definition_name: name.to_string(),
                position: placement_position,
            });
        }
    }
}

pub fn prop_tile_aligned(world_pos: Vec2, tile_size: f32, size: Vec2) -> Vec2 {
    let mut tile_world_position = Vec2 {
        x: tile_world_position(world_pos.x, tile_size),
        y: tile_world_position(world_pos.y, tile_size),
    };

    tile_world_position.x = (tile_world_position.x + size.x / 2.) - tile_size;
    tile_world_position.y = (tile_world_position.y + size.y / 2.) - tile_size;

    return tile_world_position;
}
