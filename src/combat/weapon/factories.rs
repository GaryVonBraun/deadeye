use bevy::{
    image::{ImageFilterMode, ImageLoaderSettings, ImageSampler, ImageSamplerDescriptor},
    prelude::*,
};

use crate::combat::weapon::{bundles::WeaponBundle, components::Weapon};

pub fn spawn_debug_weapon(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    translation: Vec3,
) -> Entity {
    info!("spawning weapon");
    commands
        .spawn(WeaponBundle {
            sprite: Sprite::from_image(asset_server.load_with_settings(
                "debug_weapon.png",
                |settings: &mut ImageLoaderSettings| {
                    settings.sampler = ImageSampler::Descriptor(ImageSamplerDescriptor {
                        min_filter: ImageFilterMode::Nearest,
                        mag_filter: ImageFilterMode::Nearest,
                        mipmap_filter: ImageFilterMode::Nearest,
                        ..default()
                    });
                },
            )),
            weapon: Weapon {
                fire_delay: 0.05,
                cooldown: 0.,
                speed: 2000.,
                damage: 100.,
            },
            transform: Transform::from_translation(translation),
        })
        .id()
}
