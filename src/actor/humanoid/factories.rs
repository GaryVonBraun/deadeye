use bevy::prelude::*;
use rand::RngExt;

use crate::{
    actor::{
        appearance::bundles::*,
        bundles::{AiActorBundle, CoreActorBundle},
        locomotion::components::Locomotion,
    },
    ai::bundles::AiBundle,
    combat::weapon::{component::ShootingIntent, factories::spawn_debug_weapon},
    player::components::{Player, PlayerMovementIntent},
};

// pub fn spawn_player_humanoid(
//     mut commands: Commands,
//     asset_server: Res<AssetServer>,
//     mut player_spawn_reader: MessageReader<SpawnPlayerMessage>,
// ) {
//     for message in player_spawn_reader.read() {
//         //TEMPORARY - we are spawning the weapon before the player for the moment and giving the weapon directly
//         let weapon = spawn_debug_weapon(
//             &mut commands,
//             &asset_server,
//             Vec3 {
//                 x: 10.0,
//                 y: 0.,
//                 z: 1.,
//             },
//         );

//         info!("spawning player at: {:?}", message.position);

//         let entity = commands
//             .spawn((
//                 CoreActorBundle::default_with_translation(message.position.extend(0.)),
//                 AppearanceBundle {
//                     sprite: Sprite::from_image(asset_server.load("debug_ball.png")),
//                     appearance: Appearance,
//                 },
//                 Locomotion::with_speed(100.),
//                 PlayerMovementIntent::default(),
//                 ShootingIntent::default(),
//                 Player,
//             ))
//             .add_children(&[weapon])
//             .id();
//         info!("spawned player, id: {:?}", entity);
//     }
// }

pub fn spawn_training_dummy(mut commands: Commands, asset_server: Res<AssetServer>) {
    let entity = commands
        .spawn((
            CoreActorBundle::default_with_translation(Vec3 {
                x: -50.,
                y: -50.,
                z: 0.,
            }),
            AppearanceBundle {
                sprite: Sprite::from_image(asset_server.load("debug_ball.png")),
                appearance: Appearance,
            },
        ))
        .id();
    info!("spawned training dummy, id: {:?}", entity);
}

pub fn spawn_test_ai_with_random_pos(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    let mut rng = rand::rng();
    let weapon = spawn_debug_weapon(
        commands,
        &asset_server,
        Vec3 {
            x: 0.0,
            y: 0.,
            z: 1.,
        },
    );
    let entity = commands
        .spawn((
            AiActorBundle {
                core: CoreActorBundle::default_with_translation(Vec3 {
                    x: rng.random_range(-500..500) as f32,
                    y: rng.random_range(-500..500) as f32,
                    z: 0.,
                }),
                ai: AiBundle::with_vision_range(200.),
            },
            Locomotion::with_speed(50.),
            ShootingIntent::default(),
            AppearanceBundle {
                sprite: Sprite::from_image(asset_server.load("debug_ball.png")),
                appearance: Appearance,
            },
        ))
        .add_child(weapon)
        .id();
    info!("spawned test ai, id: {:?}", entity);
}

pub fn spawn_multiple_test_ai(mut commands: Commands, asset_server: Res<AssetServer>) {
    for _ in 0..3 {
        let mut rng = rand::rng();
        let weapon = spawn_debug_weapon(
            &mut commands,
            &asset_server,
            Vec3 {
                x: 0.0,
                y: 0.,
                z: 1.,
            },
        );
        let entity = commands
            .spawn((
                AiActorBundle {
                    core: CoreActorBundle::default_with_translation(Vec3 {
                        x: rng.random_range(-700..700) as f32,
                        y: rng.random_range(-700..700) as f32,
                        z: 0.,
                    }),
                    ai: AiBundle::with_vision_range(200.),
                },
                Locomotion::with_speed(50.),
                ShootingIntent::default(),
                AppearanceBundle {
                    sprite: Sprite::from_image(asset_server.load("debug_ball.png")),
                    appearance: Appearance,
                },
            ))
            .add_child(weapon)
            .id();
        info!("spawned test ai, id: {:?}", entity);
    }
}
