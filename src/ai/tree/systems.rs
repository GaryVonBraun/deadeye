use bevy::prelude::*;

use crate::{ai::components::AiController, combat::health::components::Dead};

pub fn behavior_tree_system(mut ai_query: Query<&mut AiController, Without<Dead>>) {
    for mut ai_controller in ai_query.iter_mut() {
        let ai_controller = ai_controller.as_mut();
        ai_controller
            .action_tree
            .tick(&mut ai_controller.black_board, &mut ai_controller.intent);
        ai_controller
            .locomotion_tree
            .tick(&mut ai_controller.black_board, &mut ai_controller.intent);
    }
}
