use std::path::PathBuf;

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{
    actor::{
        appearance::bundles::{Appearance, AppearanceBundle},
        bundles::CoreActorBundle,
        components::Team,
        locomotion::components::Locomotion,
        messages::SpawnActorMessage,
    },
    ai::{
        bundles::{BaseAiBundle, SentientAiBundle},
        components::{AiController, SeekNearestTarget},
    },
    combat::{
        components::{MeleeIntent, MeleeState, ShootingIntent},
        weapon::factories::spawn_debug_weapon,
    },
    core::io::read_ron_file,
    navigation::flow_field::components::{FlowFieldNavigator, FlowFieldTarget},
    player::components::{Player, PlayerMovementIntent},
};

#[derive(Serialize, Deserialize, Debug)]
pub enum ActorArchetype {
    Player,
    Human,
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
    pub team: Team,
    pub collision_radius: f32,

    // melee stats
    pub melee_delay: f32,
    pub melee_cooldown: f32,
    pub melee_range: f32,
    pub melee_damage: f32,
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

        // info!("{:?}", message);

        match actor.archetype {
            ActorArchetype::Player => {
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
                        CoreActorBundle::from_actor_with_position(
                            message.position.extend(0.),
                            actor,
                        ),
                        AppearanceBundle {
                            sprite: Sprite::from_image(asset_server.load(actor.sprite.clone())),
                            appearance: Appearance,
                        },
                        Locomotion::with_speed(actor.speed),
                        FlowFieldTarget::default(),
                        PlayerMovementIntent::default(),
                        ShootingIntent::default(),
                        Player,
                    ))
                    .add_child(weapon)
                    .id();
            }
            ActorArchetype::Human => {
                let entity: Entity = commands
                    .spawn((
                        CoreActorBundle::from_actor_with_position(
                            message.position.extend(0.),
                            actor,
                        ),
                        SentientAiBundle::with_vision_range(actor.vision_range),
                        Locomotion::with_speed(actor.speed),
                        FlowFieldTarget::default(),
                        AppearanceBundle {
                            sprite: Sprite::from_image(asset_server.load("debug_ball.png")),
                            appearance: Appearance,
                        },
                    ))
                    .id();
            }
            ActorArchetype::Zombie => {
                let entity = commands
                    .spawn((
                        CoreActorBundle::from_actor_with_position(
                            message.position.extend(0.),
                            actor,
                        ),
                        BaseAiBundle::with_controller(AiController::zombie()),
                        Locomotion::with_speed(actor.speed),
                        SeekNearestTarget,
                        AppearanceBundle {
                            sprite: Sprite::from_image(asset_server.load(actor.sprite.clone())),
                            appearance: Appearance,
                        },
                        FlowFieldNavigator,
                        MeleeIntent {
                            target: None,
                            melee_state: MeleeState::Ready,
                            delay: actor.melee_delay,
                            cooldown: actor.melee_cooldown,
                            range: actor.melee_range,
                            damage: actor.melee_damage,
                        },
                    ))
                    .id();
            }
        }
    }
}
