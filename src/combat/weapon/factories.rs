use bevy::prelude::*;

use crate::combat::weapon::{bundles::WeaponBundle, component::Weapon};

pub fn spawn_debug_weapon(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    translation: Vec3,
) -> Entity {
    info!("spawning weapon");
    commands
        .spawn(WeaponBundle {
            sprite: Sprite::from_image(asset_server.load("debug_weapon.png")),
            weapon: Weapon {
                fire_delay: 0.4,
                cooldown: 0.,
                speed: 500.,
                damage: 10.,
            },
            transform: Transform::from_translation(translation),
        })
        .id()
}
