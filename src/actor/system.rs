use std::path::PathBuf;

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{
    actor::{
        appearance::bundles::{Appearance, AppearanceBundle},
        bundles::CoreActorBundle,
        locomotion::components::Locomotion,
        messages::SpawnActorMessage,
    },
    combat::weapon::component::ShootingIntent,
    core::io::read_ron_file,
    player::components::{Player, PlayerMovementIntent},
};

#[derive(Serialize, Deserialize, Debug)]
pub enum ActorArchetype {
    Player,
    HumanNPC,
    Zombie,
}

#[derive(Serialize, Deserialize, Debug)]
struct ActorDefinitions {
    actors: Vec<ActorDefinition>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ActorDefinition {
    pub id: String,
    pub archetype: ActorArchetype,
    pub health: f32,
    pub speed: f32,
    pub vision_range: f32,
    pub sprite: PathBuf,
}

pub fn spawn_actor_handler(
    mut spawn_actor_reader: MessageReader<SpawnActorMessage>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let Ok(definitions) =
        read_ron_file::<ActorDefinitions>(PathBuf::from("content/actors/definitions.ron"))
    else {
        error!("Failed to get actor definitions");
        return;
    };

    for message in spawn_actor_reader.read() {
        // debug message for now

        let Some(actor) = definitions
            .actors
            .iter()
            .find(|actor| actor.id == message.id)
        else {
            info!("Failed to find actor with id: {}", message.id);
            return;
        };

        info!("found actor {:?}", &actor);

        // info!("{:?}", message);

        match actor.archetype {
            ActorArchetype::Player => {
                let entity = commands
                    .spawn((
                        CoreActorBundle::from_actor_with_position(
                            message.position.extend(0.),
                            actor,
                        ),
                        AppearanceBundle {
                            sprite: Sprite::from_image(asset_server.load(actor.sprite.clone())),
                            appearance: Appearance,
                        },
                        Locomotion::with_speed(actor.speed),
                        PlayerMovementIntent::default(),
                        ShootingIntent::default(),
                        Player,
                    ))
                    .id();
            }
            ActorArchetype::HumanNPC => todo!(),
            ActorArchetype::Zombie => todo!(),
        }

        // let mut rng = rand::rng();
        // let entity2 = commands
        //     .spawn((
        //         AiActorBundle {
        //             core: CoreActorBundle::default_with_translation(Vec3 {
        //                 x: rng.random_range(-700..700) as f32,
        //                 y: rng.random_range(-700..700) as f32,
        //                 z: 0.,
        //             }),
        //             ai: AiBundle::with_vision_range(200.),
        //         },
        //         Locomotion::with_speed(50.),
        //         ShootingIntent::default(),
        //         AppearanceBundle {
        //             sprite: Sprite::from_image(asset_server.load("debug_ball.png")),
        //             appearance: Appearance,
        //         },
        //     ))
        //     .id();
    }
}
