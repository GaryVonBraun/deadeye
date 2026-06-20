use std::path::PathBuf;

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{
    actor::{
        bundles::CoreActorBundle,
        components::{Team, Zombie},
        locomotion::components::Locomotion,
        messages::SpawnActorMessage,
    },
    ai::{
        bundles::{BaseAiBundle, SentientAiBundle},
        components::{AiController, SeekNearestHostile},
    },
    animation::{components::SpriteAnimator, resources::AnimationRegistry},
    combat::{
        components::{EquippedWeapon, MeleeIntent, MeleeState, ShootingIntent},
        weapon::factories::spawn_weapon,
    },
    core::io::read_ron_file,
    navigation::flow_field::components::{FlowFieldBundle, FlowFieldTarget},
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
    animation_registry: Res<AnimationRegistry>,
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
            error!("Failed to find actor with id: {}", message.id);
            return;
        };

        let weapons: Vec<Entity> = message
            .weapons
            .iter()
            .filter_map(|id| {
                spawn_weapon(
                    id,
                    &mut commands,
                    Vec3 {
                        x: 0.0,
                        y: -4.,
                        z: 2.,
                    },
                    &animation_registry,
                )
            })
            .collect();

        let entity: Entity = match actor.archetype {
            ActorArchetype::Player => {
                let Some(anim_def) = animation_registry.entries.get("soldier_default") else {
                    error!("animation def not found");
                    return;
                };
                let Some(clip) = anim_def.clips.get(&anim_def.default) else {
                    error!("clip {} not found", anim_def.default);
                    return;
                };

                commands
                    .spawn((
                        CoreActorBundle::from_actor_with_position(
                            message.position.extend(0.),
                            actor,
                        ),
                        Sprite {
                            image: clip.image_handle.clone(),
                            texture_atlas: Some(TextureAtlas {
                                layout: clip.layout.clone(),
                                index: 0,
                            }),
                            ..default()
                        },
                        SpriteAnimator {
                            current_clip: anim_def.default.clone(),
                            frame_timer: Timer::from_seconds(
                                1.0 / clip.fps as f32,
                                TimerMode::Repeating,
                            ),
                            current_frame: 0,
                            def_id: "soldier_default".to_string(),
                            clip_dirty: false,
                            flip_x: false,
                            flip_y: false,
                        },
                        Locomotion::with_speed(actor.speed),
                        FlowFieldTarget::default(),
                        PlayerMovementIntent::default(),
                        ShootingIntent::default(),
                        Player,
                    ))
                    .add_children(&weapons)
                    .id()
            }
            ActorArchetype::Human => {
                let Some(anim_def) = animation_registry.entries.get("soldier_default") else {
                    error!("animation def not found");
                    return;
                };
                let Some(clip) = anim_def.clips.get(&anim_def.default) else {
                    error!("clip {} not found", anim_def.default);
                    return;
                };

                commands
                    .spawn((
                        CoreActorBundle::from_actor_with_position(
                            message.position.extend(0.),
                            actor,
                        ),
                        Sprite {
                            image: clip.image_handle.clone(),
                            texture_atlas: Some(TextureAtlas {
                                layout: clip.layout.clone(),
                                index: 0,
                            }),
                            ..default()
                        },
                        FlowFieldTarget::default(),
                        SentientAiBundle::with_vision_range(actor.vision_range),
                        Locomotion::with_speed(actor.speed),
                        SpriteAnimator {
                            current_clip: anim_def.default.clone(),
                            frame_timer: Timer::from_seconds(
                                1.0 / clip.fps as f32,
                                TimerMode::Repeating,
                            ),
                            current_frame: 0,
                            def_id: "soldier_default".to_string(),
                            clip_dirty: false,
                            flip_x: false,
                            flip_y: false,
                        },
                        ShootingIntent::default(),
                    ))
                    .add_children(&weapons)
                    .id()
            }
            ActorArchetype::Zombie => {
                let Some(anim_def) = animation_registry.entries.get("zombie_default") else {
                    error!("animation def not found");
                    continue;
                };

                let Some(clip) = anim_def.clips.get(&anim_def.default) else {
                    error!("default clip not found");
                    continue;
                };

                commands
                    .spawn((
                        CoreActorBundle::from_actor_with_position(
                            message.position.extend(0.),
                            actor,
                        ),
                        BaseAiBundle::with_controller(AiController::zombie()),
                        Locomotion::with_speed(actor.speed),
                        SeekNearestHostile,
                        FlowFieldBundle::default(),
                        Zombie,
                        Sprite {
                            image: clip.image_handle.clone(),
                            texture_atlas: Some(TextureAtlas {
                                layout: clip.layout.clone(),
                                index: 0,
                            }),
                            ..default()
                        },
                        SpriteAnimator {
                            current_clip: anim_def.default.clone(),
                            frame_timer: Timer::from_seconds(
                                1.0 / clip.fps as f32,
                                TimerMode::Repeating,
                            ),
                            current_frame: 0,
                            def_id: "zombie_default".to_string(),
                            clip_dirty: false,
                            flip_x: false,
                            flip_y: false,
                        },
                        MeleeIntent {
                            target: None,
                            melee_state: MeleeState::Ready,
                            delay: actor.melee_delay,
                            cooldown: actor.melee_cooldown,
                            range: actor.melee_range,
                            damage: actor.melee_damage,
                        },
                    ))
                    .id()
            }
        };
        //TEMPORARY - TODO: This is a hack to make sure the first weapon gets equipped.
        if let Some(first_weapon) = weapons.first() {
            commands.entity(entity).insert(EquippedWeapon {
                entity: *first_weapon,
            });
        }
    }
}
