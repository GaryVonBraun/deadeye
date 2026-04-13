use std::path::PathBuf;

use bevy::prelude::*;

use crate::{
    actor::components::Zombie,
    ai::components::AiMovementIntent,
    animation::{components::SpriteAnimator, resources::AnimationDefinitions},
    core::io::read_ron_file,
    player::components::{Player, PlayerMovementIntent},
};

pub fn swap_clip_texture(
    sprite: &mut Sprite,
    animator: &mut SpriteAnimator,
    animation_definitions: &AnimationDefinitions,
    asset_server: &AssetServer,
    texture_atlas_layouts: &mut Assets<TextureAtlasLayout>,
) {
    let Some(anim_def) = animation_definitions.defs.get(&animator.def_id) else {
        error!("animation def not found");
        return;
    };
    let Some(clip) = anim_def.clips.get(&animator.current_clip) else {
        error!("clip {} not found", animator.current_clip);
        return;
    };

    sprite.image = asset_server.load(&clip.texture);
    let layout = TextureAtlasLayout::from_grid(
        UVec2::new(clip.frame_size.0, clip.frame_size.1),
        clip.columns,
        clip.rows,
        None,
        None,
    );
    let layout_handle = texture_atlas_layouts.add(layout);
    if let Some(atlas) = sprite.texture_atlas.as_mut() {
        atlas.layout = layout_handle;
        atlas.index = 0;
    }
}

pub fn load_animation_definitions(mut commands: Commands) {
    info!("Loading animation definitions");

    let Ok(animation_definitions) = read_ron_file::<AnimationDefinitions>(PathBuf::from(
        "content/animation/animation_definitions.ron",
    )) else {
        error!("Failed to find animation definitions");
        return;
    };

    info!("Loaded: {:?}", animation_definitions);

    commands.insert_resource(animation_definitions);
}

pub fn sprite_animator(
    mut animator_query: Query<(&mut Sprite, &mut SpriteAnimator)>,
    animation_definitions: Res<AnimationDefinitions>,
    timer: Res<Time>,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    for (mut sprite, mut animator) in animator_query.iter_mut() {
        if animator.clip_dirty {
            animator.clip_dirty = false;
            swap_clip_texture(
                &mut sprite,
                &mut animator,
                &animation_definitions,
                &asset_server,
                &mut texture_atlas_layouts,
            );
        }

        animator.frame_timer.tick(timer.delta());
        if !animator.frame_timer.just_finished() {
            continue;
        }

        animator.current_frame += 1;

        let Some(anim_def) = animation_definitions.defs.get(&animator.def_id) else {
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
            }
        }

        sprite.flip_x = animator.flip_x;

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
    mut player_query: Query<(&PlayerMovementIntent, &mut SpriteAnimator), With<Player>>,
    animation_defs: Res<AnimationDefinitions>,
) {
    for (intent, mut animator) in player_query.iter_mut() {
        let Some(anim_def) = animation_defs.defs.get(&animator.def_id) else {
            error!("animation def not found");
            continue;
        };

        if intent.direction.x < 0.0 {
            animator.flip_x = true;
        } else if intent.direction.x > 0.0 {
            animator.flip_x = false;
        }

        let target_clip_name = if intent.direction == Vec2::default() {
            "idle"
        } else {
            "run"
        };

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
    mut zombie_query: Query<(&AiMovementIntent, &mut SpriteAnimator), With<Zombie>>,
    animation_defs: Res<AnimationDefinitions>,
) {
    for (intent, mut animator) in zombie_query.iter_mut() {
        let Some(anim_def) = animation_defs.defs.get(&animator.def_id) else {
            error!("animation def not found");
            continue;
        };

        if intent.move_direction.x < 0.0 {
            animator.flip_x = true;
        } else if intent.move_direction.x > 0.0 {
            animator.flip_x = false;
        }

        let target_clip_name = if intent.move_direction == Vec2::default() {
            "idle"
        } else {
            "walk"
        };

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
