use bevy::prelude::*;

use crate::{
    actor::{
        appearance::bundles::*, bundles::CoreActorBundle, components::Actor,
        locomotion::components::Locomotion,
    },
    combat::weapon::factories::spawn_debug_weapon,
    debug::components::DebugMovementIntent,
    player::components::{PlayerMovementIntent, PlayerShootingIntent},
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
    //TEMPORARY - we are spawning the weapon before the player for the moment and giving the weapon directly
    let weapon = spawn_debug_weapon(
        &mut commands,
        &asset_server,
        Vec3 {
            x: 10.0,
            y: 0.,
            z: 0.,
        },
    );
    commands
        .spawn((
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
            PlayerShootingIntent {
                direction: Vec2::default(),
            },
        ))
        .add_child(weapon);
    info!("spawned player");
}
