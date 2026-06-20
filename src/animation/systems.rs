use bevy::{
    image::{ImageFilterMode, ImageLoaderSettings, ImageSampler, ImageSamplerDescriptor},
    prelude::*,
};
use std::collections::HashMap;
use std::f32::consts::PI;
use std::path::PathBuf;

use crate::{
    actor::components::Zombie,
    ai::components::AiMovementIntent,
    animation::{components::*, resources::*},
    combat::{
        components::{MeleeIntent, MeleeState, ShootingIntent},
        health::components::Dead,
        weapon::components::Weapon,
    },
    core::io::read_ron_file,
    player::components::{Player, PlayerMovementIntent},
};

pub fn setup_animations(
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut commands: Commands,
) {
    info!("setting up audio");
    let Ok(animation_entries) = read_ron_file::<Vec<AnimationDefinition>>(PathBuf::from(
        "content/animation/animation_definitions.ron",
    )) else {
        error!("failed to find animation registry");
        return;
    };
    let mut registry_entries: HashMap<String, LoadedAnimation> = HashMap::new();
    for definition in animation_entries {
        let mut clips: HashMap<String, LoadedAnimationClip> = HashMap::new();

        for clip in definition.clips {
            let image: Handle<Image> = asset_server.load_with_settings(
                &clip.texture,
                |settings: &mut ImageLoaderSettings| {
                    settings.sampler = ImageSampler::Descriptor(ImageSamplerDescriptor {
                        min_filter: ImageFilterMode::Nearest,
                        mag_filter: ImageFilterMode::Nearest,
                        mipmap_filter: ImageFilterMode::Nearest,
                        ..default()
                    });
                },
            );
            let layout = TextureAtlasLayout::from_grid(
                UVec2::new(clip.frame_size.0, clip.frame_size.1),
                clip.columns,
                clip.rows,
                None,
                None,
            );
            let layout_handle = texture_atlas_layouts.add(layout);
            // if let Some(atlas) = sprite.texture_atlas.as_mut() {
            //     atlas.layout = layout_handle;
            //     atlas.index = 0;
            // }

            clips.insert(
                clip.name,
                LoadedAnimationClip {
                    image_handle: image,
                    layout: layout_handle,
                    frame_size: clip.frame_size,
                    columns: clip.columns,
                    rows: clip.rows,
                    fps: clip.fps,
                    looping: clip.looping,
                    freeze: clip.freeze,
                },
            );
        }
        let animation_entry: LoadedAnimation = LoadedAnimation {
            default: definition.default,
            clips,
        };
        registry_entries.insert(definition.name, animation_entry);
    }
    commands.insert_resource(AnimationRegistry {
        entries: registry_entries,
    });
}

pub fn swap_clip_texture(
    sprite: &mut Sprite,
    animator: &mut SpriteAnimator,
    animation_registry: &AnimationRegistry,
) {
    let Some(anim_def) = animation_registry.entries.get(&animator.def_id) else {
        error!("animation def not found");
        return;
    };
    let Some(clip) = anim_def.clips.get(&animator.current_clip) else {
        error!("clip {} not found", animator.current_clip);
        return;
    };

    // info!("swapping texture");

    sprite.image = clip.image_handle.clone();

    if let Some(atlas) = sprite.texture_atlas.as_mut() {
        atlas.layout = clip.layout.clone();
        atlas.index = 0;
    }
}
pub fn sprite_animator(
    mut animator_query: Query<(Entity, &mut Sprite, &mut SpriteAnimator)>,
    animation_definitions: Res<AnimationRegistry>,
    timer: Res<Time>,
    mut commands: Commands,
) {
    for (entity, mut sprite, mut animator) in animator_query.iter_mut() {
        sprite.flip_x = animator.flip_x;
        sprite.flip_y = animator.flip_y;
        if animator.clip_dirty {
            animator.clip_dirty = false;
            swap_clip_texture(&mut sprite, &mut animator, &animation_definitions);
        }

        animator.frame_timer.tick(timer.delta());
        if !animator.frame_timer.just_finished() {
            continue;
        }

        animator.current_frame += 1;

        let Some(anim_def) = animation_definitions.entries.get(&animator.def_id) else {
            error!("animation def not found");
            continue;
        };

        let Some(clip) = anim_def.clips.get(&animator.current_clip) else {
            error!("default clip not found");
            continue;
        };

        let last_frame = (clip.columns * clip.rows) as usize - 1;
        if animator.current_frame > last_frame {
            if clip.looping {
                animator.current_frame = 0;
            } else {
                animator.current_frame -= 1;
                commands.entity(entity).insert(AnimationFinished);
            }
        }

        sprite.texture_atlas.as_mut().unwrap().index = animator.current_frame;
        animator.frame_timer = Timer::from_seconds(1.0 / clip.fps as f32, TimerMode::Once);
    }
}

pub fn set_clip(animator: &mut SpriteAnimator, clip_name: &str, fps: f32) {
    if animator.current_clip == clip_name {
        return;
    } // already playing
    animator.current_clip = clip_name.to_string();
    animator.current_frame = 0;
    animator.frame_timer = Timer::from_seconds(1.0 / fps, TimerMode::Once);
    animator.clip_dirty = true;
}

pub fn player_animation_state(
    mut player_query: Query<
        (
            &Transform,
            &PlayerMovementIntent,
            Option<&ShootingIntent>,
            &mut SpriteAnimator,
            Option<&Dead>,
        ),
        With<Player>,
    >,
    animation_defs: Res<AnimationRegistry>,
) {
    for (transform, movement_intent, shooting_intent, mut animator, dead) in player_query.iter_mut()
    {
        let Some(anim_def) = animation_defs.entries.get(&animator.def_id) else {
            error!("animation def not found");
            continue;
        };

        if let Some(intent) = shooting_intent {
            let direction = (intent.target_position - transform.translation.truncate()).normalize();

            animator.flip_x = direction.x < 0.0;
        } else {
            animator.flip_x = movement_intent.direction.x < 0.0;
        }

        let mut target_clip_name = if movement_intent.direction == Vec2::default() {
            "idle"
        } else {
            "run"
        };

        if dead != None {
            target_clip_name = "dead";
        }

        if animator.current_clip == target_clip_name {
            continue;
        }

        let Some(target_clip) = anim_def.clips.get(target_clip_name) else {
            error!("clip {} not found", target_clip_name);
            continue;
        };

        set_clip(&mut animator, target_clip_name, target_clip.fps);
    }
}

pub fn zombie_animation_state(
    mut zombie_query: Query<
        (
            &AiMovementIntent,
            &mut SpriteAnimator,
            &MeleeIntent,
            Option<&Dead>,
        ),
        With<Zombie>,
    >,
    animation_defs: Res<AnimationRegistry>,
) {
    for (intent, mut animator, melee_intent, dead) in zombie_query.iter_mut() {
        let Some(anim_def) = animation_defs.entries.get(&animator.def_id) else {
            error!("animation def not found");
            continue;
        };

        let mut target_clip_name = if intent.move_direction == Vec2::default() {
            "idle"
        } else {
            "walk"
        };

        match melee_intent.melee_state {
            MeleeState::AttackDelay(_) => target_clip_name = "attack",
            _ => {}
        }
        if dead != None {
            target_clip_name = "dead";
        }

        if intent.move_direction.x < 0.0 {
            animator.flip_x = true;
        } else if intent.move_direction.x > 0.0 {
            animator.flip_x = false;
        }

        if animator.current_clip == target_clip_name {
            continue;
        }

        let Some(target_clip) = anim_def.clips.get(target_clip_name) else {
            error!("clip {} not found", target_clip_name);
            continue;
        };

        set_clip(&mut animator, target_clip_name, target_clip.fps);
    }
}

pub fn weapon_animation_state(
    mut weapon_query: Query<(&mut SpriteAnimator, &Transform), With<Weapon>>,
    // animation_defs: Res<AnimationRegistry>,
) {
    for (mut animator, transform) in weapon_query.iter_mut() {
        // let Some(anim_def) = animation_defs.defs.get(&animator.def_id) else {
        //     error!("animation def not found");
        //     continue;
        // };

        let (_, _, angle) = transform.rotation.to_euler(EulerRot::XYZ);

        // Flip when facing left (angle beyond +/- 90 degrees)
        if angle > PI / 2.0 || angle < -PI / 2.0 {
            animator.flip_y = true;
        } else {
            animator.flip_y = false;
        }

        // if animator.current_clip == target_clip_name {
        //     continue;
        // }

        // let Some(target_clip) = anim_def.clips.get(target_clip_name) else {
        //     error!("clip {} not found", target_clip_name);
        //     continue;
        // };

        // set_clip(&mut animator, target_clip_name, target_clip.fps);
    }
}
