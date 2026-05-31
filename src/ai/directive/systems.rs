use bevy::prelude::*;

use crate::{
    ai::{components::AiController, directive::components::AiDirective},
    player::components::Player,
};

//TEMPORARY - system that turns all actors with directive to follow player
pub fn set_follow_player_directive(
    player_query: Query<Entity, With<Player>>,
    mut directive_query: Query<&mut AiDirective>,
) {
    let Ok(player) = player_query.single() else {
        return;
    };

    for mut directive in directive_query.iter_mut() {
        *directive = AiDirective::Follow(player);
    }
}

pub fn update_blackboard_directive(mut directive_query: Query<(&mut AiController, &AiDirective)>) {
    for (mut controller, directive) in directive_query.iter_mut() {
        if controller.black_board.directive == *directive {
            return;
        }
        controller.black_board.directive = directive.clone();
    }
}
