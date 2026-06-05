use bevy::prelude::*;

use crate::{
    animation::{components::SpriteAnimator, resources::AnimationRegistry},
    combat::weapon::{
        bundles::WeaponBundle,
        components::{FireMode, Weapon, WeaponRuntime},
    },
};

pub fn spawn_debug_weapon(
    commands: &mut Commands,
    translation: Vec3,
    animation_registry: &Res<AnimationRegistry>,
) -> Entity {
    info!("spawning weapon");

    let weapon_config = Weapon {
        fire_delay: 0.1,
        reload_time: 3.,
        magazine_size: 30,
        speed: 500.,
        damage: 100.,
        shoot_sound: "weapon_pistol_fire".to_string(),
        reload_sound: "weapon_ak_reload".to_string(),
        dry_sound: "weapon_pistol_dry".to_string(),
        fire_mode: FireMode::Auto,
        // spread
        spread_base: 0.,
        spread_max: 0.3,
        spread_per_shot: 0.2,
        spread_recovery: 1.,
        movement_spread: 0.2,
    };

    //FIXME - Its currently being duplicated but its temporary anyways
    let Some(anim_def) = animation_registry.entries.get("weapon_default") else {
        error!("animation def not found");
        return commands
            .spawn((
                WeaponRuntime::new_with_ammo(weapon_config.magazine_size),
                weapon_config,
                Transform::from_translation(translation),
            ))
            .id();
    };

    let Some(clip) = anim_def.clips.get(&anim_def.default) else {
        error!("clip {} not found", anim_def.default);
        return commands
            .spawn((
                WeaponRuntime::new_with_ammo(weapon_config.magazine_size),
                weapon_config,
                Transform::from_translation(translation),
            ))
            .id();
    };

    commands
        .spawn(WeaponBundle {
            weapon_runtime: WeaponRuntime::new_with_ammo(weapon_config.magazine_size),
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
            weapon: weapon_config,
            transform: Transform::from_translation(translation),
        })
        .id()
}

pub fn spawn_pistole_weapon(
    commands: &mut Commands,
    translation: Vec3,
    animation_registry: &Res<AnimationRegistry>,
) -> Entity {
    info!("spawning weapon");

    let weapon_config = Weapon {
        fire_delay: 0.5,
        reload_time: 3.,
        magazine_size: 15,
        speed: 500.,
        damage: 15.,
        shoot_sound: "weapon_pistol_fire".to_string(),
        reload_sound: "weapon_ak_reload".to_string(),
        dry_sound: "weapon_pistol_dry".to_string(),
        fire_mode: FireMode::Semi,
        // spread
        spread_base: 1.,
        spread_max: 30.,
        spread_per_shot: 2.,
        spread_recovery: 1.,
        movement_spread: 10.,
    };

    //FIXME - Its currently being duplicated but its temporary anyways
    let Some(anim_def) = animation_registry.entries.get("weapon_default") else {
        error!("animation def not found");
        return commands
            .spawn((
                WeaponRuntime::new_with_ammo(weapon_config.magazine_size),
                weapon_config,
                Transform::from_translation(translation),
            ))
            .id();
    };

    let Some(clip) = anim_def.clips.get(&anim_def.default) else {
        error!("clip {} not found", anim_def.default);
        return commands
            .spawn((
                WeaponRuntime::new_with_ammo(weapon_config.magazine_size),
                weapon_config,
                Transform::from_translation(translation),
            ))
            .id();
    };

    commands
        .spawn(WeaponBundle {
            weapon_runtime: WeaponRuntime::new_with_ammo(weapon_config.magazine_size),
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
            weapon: weapon_config,
            transform: Transform::from_translation(translation),
        })
        .id()
}
