use bevy::prelude::*;

use crate::combat::weapon::{bundles::WeaponBundle, component::Weapon};

pub fn spawn_debug_weapon(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    info!("spawning weapon");
    commands
        .spawn(WeaponBundle {
            sprite: Sprite::from_image(asset_server.load("debug_weapon.png")),
            weapon: Weapon {
                fire_rate: 1.,
                projectile_speed: 10.,
            },
            transform: Transform::default()
        })
        .id()
}
