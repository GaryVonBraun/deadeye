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
    mut weapon_query: Query<(&mut Weapon, &GlobalTransform), With<Weapon>>,
    asset_server: Res<AssetServer>,
) {
    for message in messages.read() {
        // get the children of the shooter entity
        if let Ok(children) = children_query.get(message.owner) {
            for child in children.iter() {
                // and we check if the child is a weapon
                if let Ok((mut weapon, global_transform)) = weapon_query.get_mut(child) {
                    //NOTE - currently this means if an actor has multiple weapons they all fire

                    if weapon.cooldown > 0.0 {
                        return;
                    }

                    let angle = message.direction.y.atan2(message.direction.x);
                    let rotation = Quat::from_rotation_z(angle);

                    commands.spawn(ProjectileBundle {
                        projectile: Projectile {
                            speed: weapon.speed,
                            direction: message.direction,
                            lifetime: 3.,
                            damage: weapon.damage,
                            owner: message.owner
                        },
                        sprite: Sprite::from_image(asset_server.load("debug_bullet.png")),
                        transform: Transform {
                            rotation: rotation,
                            translation: global_transform.translation(),
                            scale: Vec3::ONE,
                        },
                    });

                    weapon.cooldown = weapon.fire_delay;
                }
            }
        }
    }
}

pub fn weapon_cooldown(time: Res<Time>, mut query: Query<&mut Weapon>) {
    for mut weapon in query.iter_mut() {
        if weapon.cooldown > 0.0 {
            weapon.cooldown -= time.delta_secs();
        };
    }
}
