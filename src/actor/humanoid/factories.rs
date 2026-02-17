use bevy::prelude::*;

use crate::{
    actor::{
        appearance::bundles::*, bundles::{Actor, CoreActorBundle}, locomotion::components::Locomotion
    },
    debug::components::DebugMovementIntent,
    player::components::PlayerMovementIntent,
};

pub fn spawn_debug_humanoid(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        CoreActorBundle {
            actor: Actor,
            transform: Transform::from_xyz(0., 0., 0.),
        },
        AppearanceBundle {
            sprite: Sprite::from_image(asset_server.load("debug_ball.png")),
            appearance: Appearance,
        },
        Locomotion {
            move_direction: vec2(0., 0.),
            speed: 100.,
        },
        DebugMovementIntent {
            direction: Vec2::default(),
        },
    ));
    info!("spawned basic humanoid entity");
}

pub fn spawn_player_humanoid(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        CoreActorBundle {
            actor: Actor,
            transform: Transform::from_xyz(0., 0., 0.),
        },
        AppearanceBundle {
            sprite: Sprite::from_image(asset_server.load("debug_ball.png")),
            appearance: Appearance,
        },
        Locomotion {
            move_direction: vec2(0., 0.),
            speed: 100.,
        },
        PlayerMovementIntent {
            direction: Vec2::default(),
        },
    ));
    info!("spawned player");
}
