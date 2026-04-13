use std::path::PathBuf;

use bevy::prelude::*;

use crate::{
    animation::{components::SpriteAnimator, resources::AnimationDefinitions},
    core::io::read_ron_file,
};

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
) {
    for (mut sprite, mut animator) in animator_query.iter_mut() {
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

        sprite.texture_atlas.as_mut().unwrap().index = animator.current_frame;
        animator.frame_timer = Timer::from_seconds(1.0 / clip.fps as f32, TimerMode::Once);
    }
}
