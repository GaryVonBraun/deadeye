use bevy::prelude::*;

use crate::{
    editor::{
        component::PlacementPreview,
        resources::{EditorSettings, ToolAction},
    },
    map::{resources::ActiveMap, utility::tile_world_position},
    props::messages::{RemovePropMessage, SpawnPropMessage},
};

pub fn prop_tool_system(
    action: &ToolAction,
    world_pos: Vec2,
    editor_settings: Res<EditorSettings>,
    preview: &PlacementPreview,
    active_map: ResMut<ActiveMap>,
    mut place_prop_writer: MessageWriter<SpawnPropMessage>,
    mut remove_prop_writer: MessageWriter<RemovePropMessage>,
) {
    match action {
        ToolAction::Place(name) => {
            let placement_position: Vec2;

            if editor_settings.snap_to_grid || editor_settings.tile_aligned {
                placement_position =
                    prop_tile_aligned(world_pos, active_map.tileset.tile_size, preview.size);
            } else {
                placement_position = world_pos;
            }

            place_prop_writer.write(SpawnPropMessage {
                name: name.to_string(),
                position: placement_position,
            });
        }
        ToolAction::Remove => {
            remove_prop_writer.write(RemovePropMessage {
                position: world_pos,
            });
        }
    }
}

pub fn prop_tile_aligned(world_pos: Vec2, tile_size: f32, size: Vec2) -> Vec2 {
    let mut tile_world_position = tile_world_position(world_pos, tile_size);

    tile_world_position.x = (tile_world_position.x + size.x / 2.) - tile_size;
    tile_world_position.y = (tile_world_position.y + size.y / 2.) - tile_size;

    return tile_world_position;
}
