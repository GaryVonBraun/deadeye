use bevy::prelude::*;

use crate::{editor::resources::ToolAction, props::messages::SpawnPropMessage};

pub fn prop_tool_system(
    action: &ToolAction,
    world_pos: Vec2,
    mut place_prop_writer: MessageWriter<SpawnPropMessage>,
) {
    match action {
        ToolAction::Place(name) => {
            place_prop_writer.write(SpawnPropMessage {
                name: name.to_string(),
                position: world_pos,
            });
        }
    }
}
