use bevy::prelude::*;

use crate::{actor::{
    bundles::{Actor, CoreActorBundle},
    humanoid::appearance::bundles::*, locomotion::components::Locomotion,
}, debug::components::DebugMovementIntent};

pub fn spawn_basic_humanoid(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        CoreActorBundle {
            actor: Actor,
            transform: Transform::from_xyz(0., 0., 0.),
        },
        HumanoidAppearanceBundle {
            sprite: Sprite::from_image(asset_server.load("debug_ball.png")),
            appearance: HumanoidAppearance,
        },
        Locomotion {
            move_direction: vec2(0., 0.),
            speed: 100.
        },
        DebugMovementIntent {
            intent: Vec2::default()
        }
    ));
    info!("spawned basic humanoid entity");
}
