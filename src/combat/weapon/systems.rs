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
    weapon_query: Query<(&mut Weapon, &GlobalTransform), With<Weapon>>,
    asset_server: Res<AssetServer>,
) {
    for message in messages.read() {
        // get the children of the shooter entity
        if let Ok(children) = children_query.get(message.shooter) {
            for child in children.iter() {
                // and we check if the child is a weapon
                if let Ok((mut weapon, global_transform)) = weapon_query.get(child) {
                    //NOTE - currently this means if an actor has multiple weapons they all fire
                    info!(
                        "weapon shot with a projectile speed of {:?}",
                        weapon.projectile_speed
                    );

                    let projectile = commands
                        .spawn(ProjectileBundle {
                            projectile: Projectile { speed: weapon.projectile_speed, direction: message.direction },
                            sprite: Sprite::from_image(asset_server.load("debug_bullet.png")),
                            transform: Transform::from_translation(global_transform.translation()),
                        })
                        .id();
                    info!("projectile spawned {:?}", projectile)
                }
            }
        }
    }
}
