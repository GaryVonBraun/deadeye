use bevy::{
    image::{ImageFilterMode, ImageLoaderSettings, ImageSampler, ImageSamplerDescriptor},
    prelude::*,
};

use crate::{
    animation::{components::SpriteAnimator, resources::AnimationDefinitions},
    combat::weapon::{
        bundles::WeaponBundle,
        components::{FireMode, Weapon, WeaponRuntime, WeaponState},
    },
};

pub fn spawn_debug_weapon(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    translation: Vec3,
    animation_definitions: &Res<AnimationDefinitions>,
    texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
) -> Entity {
    info!("spawning weapon");

    let weapon_config = Weapon {
        fire_delay: 0.1,
        reload_time: 3.,
        magazine_size: 300,
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

    let Some(anim_def) = animation_definitions.defs.get("weapon_default") else {
        error!("animation def not found");
        return commands
            .spawn((
                WeaponRuntime::new_with_ammo(weapon_config.magazine_size),
                weapon_config,
                Transform::from_translation(translation),
            ))
            .id();
    };
    let default_clip_name = anim_def.default.clone();

    let Some(clip) = anim_def.clips.get(&default_clip_name) else {
        error!("clip {} not found", default_clip_name);

        return commands
            .spawn((
                WeaponRuntime::new_with_ammo(weapon_config.magazine_size),
                weapon_config,
                Transform::from_translation(translation),
            ))
            .id();
    };

    let layout = TextureAtlasLayout::from_grid(
        UVec2::new(clip.frame_size.0, clip.frame_size.1),
        clip.columns as u32,
        clip.rows as u32,
        None,
        None,
    );
    let layout_handle = texture_atlas_layouts.add(layout);

    commands
        .spawn(WeaponBundle {
            weapon_runtime: WeaponRuntime::new_with_ammo(weapon_config.magazine_size),
            sprite: Sprite {
                image: asset_server.load_with_settings(
                    &clip.texture,
                    |settings: &mut ImageLoaderSettings| {
                        settings.sampler = ImageSampler::Descriptor(ImageSamplerDescriptor {
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
            sprite_animator: SpriteAnimator {
                current_clip: default_clip_name,
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
    asset_server: &Res<AssetServer>,
    translation: Vec3,
    animation_definitions: &Res<AnimationDefinitions>,
    texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
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

    let Some(anim_def) = animation_definitions.defs.get("weapon_default") else {
        error!("animation def not found");
        return commands
            .spawn((
                WeaponRuntime::new_with_ammo(weapon_config.magazine_size),
                weapon_config,
                Transform::from_translation(translation),
            ))
            .id();
    };
    let default_clip_name = anim_def.default.clone();

    let Some(clip) = anim_def.clips.get(&default_clip_name) else {
        error!("clip {} not found", default_clip_name);

        return commands
            .spawn((
                WeaponRuntime::new_with_ammo(weapon_config.magazine_size),
                weapon_config,
                Transform::from_translation(translation),
            ))
            .id();
    };

    let layout = TextureAtlasLayout::from_grid(
        UVec2::new(clip.frame_size.0, clip.frame_size.1),
        clip.columns as u32,
        clip.rows as u32,
        None,
        None,
    );
    let layout_handle = texture_atlas_layouts.add(layout);

    commands
        .spawn(WeaponBundle {
            weapon_runtime: WeaponRuntime::new_with_ammo(weapon_config.magazine_size),
            sprite: Sprite {
                image: asset_server.load_with_settings(
                    &clip.texture,
                    |settings: &mut ImageLoaderSettings| {
                        settings.sampler = ImageSampler::Descriptor(ImageSamplerDescriptor {
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
            sprite_animator: SpriteAnimator {
                current_clip: default_clip_name,
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
