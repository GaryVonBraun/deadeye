use bevy::prelude::*;

use crate::{
    actor::components::Actor,
    combat::{
        messages::ShootMessage,
        projectiles::{bundles::ProjectileBundle, component::Projectile},
        weapon::component::Weapon,
    },
};

pub fn shoot_weapon(
    mut commands: Commands,
    mut messages: MessageReader<ShootMessage>,
    children_query: Query<&Children, With<Actor>>,
    weapon_query: Query<(&Weapon, &GlobalTransform), With<Weapon>>,
    asset_server: Res<AssetServer>,
) {
    for message in messages.read() {
        // get the children of the shooter entity
        if let Ok(children) = children_query.get(message.shooter) {
            for child in children.iter() {
                // and we check if the child is a weapon
                if let Ok((weapon, global_transform)) = weapon_query.get(child) {
                    //NOTE - currently this means if an actor has multiple weapons they all fire
                    info!(
                        "weapon shot with a projectile speed of {:?}",
                        weapon.projectile_speed
                    );

                    let angle = message.direction.y.atan2(message.direction.x);
                    let rotation = Quat::from_rotation_z(angle);

                    let projectile = commands
                        .spawn(ProjectileBundle {
                            projectile: Projectile {
                                speed: weapon.projectile_speed,
                                direction: message.direction,
                            },
                            sprite: Sprite::from_image(asset_server.load("debug_bullet.png")),
                            transform: Transform {
                                rotation: rotation,
                                translation: global_transform.translation(),
                                scale: Vec3::ONE,
                            },
                        })
                        .id();
                    info!("projectile spawned {:?}", projectile)
                }
            }
        }
    }
}