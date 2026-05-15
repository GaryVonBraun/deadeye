use std::path::PathBuf;

use bevy::{
    image::{ImageFilterMode, ImageLoaderSettings, ImageSampler, ImageSamplerDescriptor},
    prelude::*,
};
use serde::{Deserialize, Serialize};

use crate::{
    actor::{
        appearance::bundles::{Appearance, AppearanceBundle},
        bundles::CoreActorBundle,
        components::{Team, Zombie},
        locomotion::components::Locomotion,
        messages::SpawnActorMessage,
    },
    ai::{
        bundles::{BaseAiBundle, SentientAiBundle},
        components::{AiController, SeekNearestTarget},
    },
    animation::{components::SpriteAnimator, resources::AnimationDefinitions},
    combat::{
        components::{EquippedWeapon, MeleeIntent, MeleeState, ShootingIntent},
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
    animation_definitions: Res<AnimationDefinitions>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
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
                        y: -4.,
                        z: 2.,
                    },
                    &animation_definitions,
                    &mut texture_atlas_layouts,
                );

                let weapon2 = spawn_debug_weapon(
                    &mut commands,
                    &asset_server,
                    Vec3 {
                        x: 1.0,
                        y: 4.,
                        z: 2.,
                    },
                    &animation_definitions,
                    &mut texture_atlas_layouts,
                );

                let Some(anim_def) = animation_definitions.defs.get("soldier_default") else {
                    error!("animation def not found");
                    continue;
                };

                let default_clip_name = &anim_def.default;
                let Some(clip) = anim_def.clips.get(default_clip_name) else {
                    error!("default clip not found");
                    continue;
                };

                let layout = TextureAtlasLayout::from_grid(
                    UVec2::new(clip.frame_size.0, clip.frame_size.1),
                    clip.columns as u32,
                    clip.rows as u32,
                    None,
                    None,
                );
                let layout_handle = texture_atlas_layouts.add(layout);

                let entity = commands
                    .spawn((
                        CoreActorBundle::from_actor_with_position(
                            message.position.extend(0.),
                            actor,
                        ),
                        EquippedWeapon { entity: weapon },
                        Sprite {
                            image: asset_server.load_with_settings(
                                &clip.texture,
                                |settings: &mut ImageLoaderSettings| {
                                    settings.sampler =
                                        ImageSampler::Descriptor(ImageSamplerDescriptor {
                                            min_filter: ImageFilterMode::Nearest,
                                            mag_filter: ImageFilterMode::Nearest,
                                            mipmap_filter: ImageFilterMode::Nearest,
                                            ..default()
                                        });
                                },
                            ),
                            texture_atlas: Some(TextureAtlas {
                                layout: layout_handle,
                                index: 0,
                            }),
                            ..default()
                        },
                        SpriteAnimator {
                            current_clip: default_clip_name.clone(),
                            frame_timer: Timer::from_seconds(
                                1.0 / clip.fps as f32,
                                TimerMode::Repeating,
                            ),
                            current_frame: 0,
                            def_id: "soldier_default".to_string(),
                            clip_dirty: false,
                            flip_x: false,
                        },
                        Locomotion::with_speed(actor.speed),
                        FlowFieldTarget::default(),
                        PlayerMovementIntent::default(),
                        ShootingIntent::default(),
                        Player,
                    ))
                    .add_children(&[weapon, weapon2])
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
                let Some(anim_def) = animation_definitions.defs.get("zombie_default") else {
                    error!("animation def not found");
                    continue;
                };

                let default_clip_name = &anim_def.default;
                let Some(clip) = anim_def.clips.get(default_clip_name) else {
                    error!("default clip not found");
                    continue;
                };

                let layout = TextureAtlasLayout::from_grid(
                    UVec2::new(clip.frame_size.0, clip.frame_size.1),
                    clip.columns as u32,
                    clip.rows as u32,
                    None,
                    None,
                );
                let layout_handle = texture_atlas_layouts.add(layout);

                let entity = commands
                    .spawn((
                        CoreActorBundle::from_actor_with_position(
                            message.position.extend(0.),
                            actor,
                        ),
                        BaseAiBundle::with_controller(AiController::zombie()),
                        Locomotion::with_speed(actor.speed),
                        SeekNearestTarget,
                        FlowFieldNavigator,
                        Zombie,
                        Sprite {
                            image: asset_server.load_with_settings(
                                &clip.texture,
                                |settings: &mut ImageLoaderSettings| {
                                    settings.sampler =
                                        ImageSampler::Descriptor(ImageSamplerDescriptor {
                                            min_filter: ImageFilterMode::Nearest,
                                            mag_filter: ImageFilterMode::Nearest,
                                            mipmap_filter: ImageFilterMode::Nearest,
                                            ..default()
                                        });
                                },
                            ),
                            texture_atlas: Some(TextureAtlas {
                                layout: layout_handle,
                                index: 0,
                            }),
                            ..default()
                        },
                        SpriteAnimator {
                            current_clip: default_clip_name.clone(),
                            frame_timer: Timer::from_seconds(
                                1.0 / clip.fps as f32,
                                TimerMode::Repeating,
                            ),
                            current_frame: 0,
                            def_id: "zombie_default".to_string(),
                            clip_dirty: false,
                            flip_x: false,
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
                    .id();
            }
        }
    }
}
