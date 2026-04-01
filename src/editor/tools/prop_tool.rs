use bevy::prelude::*;

use crate::{
    editor::resources::{SelectedProp, ToolAction},
    props::messages::SpawnPropMessage,
};

pub fn prop_tool_system(
    action: ToolAction,
    world_pos: Vec2,
    mut place_prop_writer: MessageWriter<SpawnPropMessage>,
    selected_prop: Res<SelectedProp>,
) {
    match action {
        ToolAction::Place => {
            place_prop_writer.write(SpawnPropMessage {
                name: selected_prop.name.clone(),
                position: world_pos,
            });
        }
    }
}
