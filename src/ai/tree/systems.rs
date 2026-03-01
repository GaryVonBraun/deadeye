use bevy::prelude::*;

use crate::ai::components::AiController;

pub fn behavior_tree_system(
    mut ai_query: Query<&mut AiController>,
) {
    for mut controller in ai_query.iter_mut() {
        let controller = controller.as_mut();
        controller.tree.tick(&mut controller.black_board);
    }
}