use bevy::{
    image::{ImageFilterMode, ImageLoaderSettings, ImageSampler, ImageSamplerDescriptor},
    prelude::*,
};

use crate::{
    animation::{components::SpriteAnimator, resources::AnimationDefinitions},
    combat::weapon::{
        bundles::WeaponBundle,
        components::{Weapon, WeaponRuntime, WeaponState},
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
        magazine_size: 300000,
        speed: 500.,
        damage: 100.,
    };

    let Some(anim_def) = animation_definitions.defs.get("weapon_default") else {
        error!("animation def not found");
        return commands
            .spawn((
                WeaponRuntime {
                    state: WeaponState::Ready,
                    ammo: weapon_config.magazine_size.clone(),
                },
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
                WeaponRuntime {
                    state: WeaponState::Ready,
                    ammo: weapon_config.magazine_size.clone(),
                },
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
            weapon_runtime: WeaponRuntime {
                state: WeaponState::Ready,
                ammo: weapon_config.magazine_size.clone(),
            },
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
            },
            weapon: weapon_config,
            transform: Transform::from_translation(translation),
        })
        .id()
}
