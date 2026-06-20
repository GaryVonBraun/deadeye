use bevy::prelude::*;

use crate::{
    animation::{components::SpriteAnimator, resources::AnimationRegistry},
    combat::weapon::{
        bundles::WeaponBundle, components::WeaponRuntime, io::operations::read_weapon_definitions,
    },
};
pub fn spawn_weapon(
    weapon_id: String,
    commands: &mut Commands,
    translation: Vec3,
    animation_registry: &Res<AnimationRegistry>,
) -> Option<Entity> {
    let Ok(definitions) = read_weapon_definitions() else {
        error!("Failed to load weapon definitions, needed for spawning weapons.");
        return None;
    };

    let Some(weapon) = definitions
        .weapons
        .iter()
        .find(|definition| definition.id == weapon_id)
    else {
        error!("Could not find a weapon with id: {}", weapon_id);
        return None;
    };

    //NOTE - This could be an animation helper function.
    let Some(anim_def) = animation_registry.entries.get("weapon_default") else {
        error!("animation def not found");
        return None;
    };

    let Some(clip) = anim_def.clips.get(&anim_def.default) else {
        error!("clip {} not found", anim_def.default);
        return None;
    };

    let weapon_entity = commands
        .spawn(WeaponBundle {
            weapon_runtime: WeaponRuntime::new_with_ammo(weapon.magazine_size),
            sprite: Sprite {
                image: clip.image_handle.clone(),
                texture_atlas: Some(TextureAtlas {
                    layout: clip.layout.clone(),
                    index: 0,
                }),
                ..default()
            },
            sprite_animator: SpriteAnimator {
                current_clip: anim_def.default.clone(),
                frame_timer: Timer::from_seconds(1.0 / clip.fps as f32, TimerMode::Repeating),
                current_frame: 0,
                def_id: "weapon_default".to_string(),
                clip_dirty: false,
                flip_x: false,
                flip_y: false,
            },
            weapon: weapon.clone(),
            transform: Transform::from_translation(translation),
        })
        .id();

    Some(weapon_entity)
}
