use bevy::{ecs::relationship::Relationship, prelude::*};

use crate::{
    actor::components::Actor,
    audio::resources::AudioRegistry,
    collision::components::CollisionShape,
    combat::{
        components::{EquippedWeapon, ShootingIntent},
        health::components::Hitbox,
        messages::{ReloadMessage, ShootMessage},
        projectiles::{bundles::ProjectileBundle, component::Projectile},
        weapon::components::{FireMode, Weapon, WeaponRuntime, WeaponState},
    },
    core::components::GameEntity,
};

pub fn shoot_weapon(
    mut commands: Commands,
    mut messages: MessageReader<ShootMessage>,
    children_query: Query<(&Children, &EquippedWeapon), With<Actor>>,
    mut weapon_query: Query<(&Weapon, &mut WeaponRuntime, &GlobalTransform), With<Weapon>>,
    asset_server: Res<AssetServer>,
    audio_registry: Res<AudioRegistry>,
) {
    for message in messages.read() {
        // get the children of the shooter entity
        if let Ok((children, equipped_weapon)) = children_query.get(message.owner) {
            for child in children.iter() {
                // and we check if the child is a weapon

                if child != equipped_weapon.entity {
                    continue;
                }

                if let Ok((weapon, mut weapon_runtime, global_transform)) =
                    weapon_query.get_mut(child)
                {
                    //NOTE - currently this means if an actor has multiple weapons they all fire

                    if weapon_runtime.state != WeaponState::Ready {
                        return;
                    }

                    match weapon.fire_mode {
                        FireMode::Semi => {
                            if !message.just_pressed {
                                return;
                            }
                        }
                        FireMode::Auto => {}
                    }

                    if weapon_runtime.ammo == 0 {
                        if message.just_pressed {
                            let Some(asset) = audio_registry.sounds.get(&weapon.dry_sound) else {
                                return;
                            };

                            commands.spawn((
                                AudioPlayer::new(asset.clone()),
                                PlaybackSettings::DESPAWN,
                            ));
                        }
                        return;
                    }

                    let direction = (message.target_position
                        - global_transform.translation().truncate())
                    .normalize();

                    let angle = direction.y.atan2(direction.x);
                    let rotation = Quat::from_rotation_z(angle);

                    let mut translation = global_transform.translation();

                    translation.z = 1.;

                    let Some(asset) = audio_registry.sounds.get(&weapon.shoot_sound) else {
                        return;
                    };

                    commands.spawn((AudioPlayer::new(asset.clone()), PlaybackSettings::DESPAWN));

                    commands.spawn(ProjectileBundle {
                        projectile: Projectile {
                            speed: weapon.speed,
                            direction: direction,
                            lifetime: 3.,
                            damage: weapon.damage,
                            owner: message.owner,
                        },
                        hitbox: Hitbox {
                            shape: CollisionShape::Circle { radius: 2. },
                            offset: Vec2::default(),
                        },
                        sprite: Sprite::from_image(asset_server.load("Gun_Bullet.png")),
                        transform: Transform {
                            rotation: rotation,
                            translation: translation,
                            scale: Vec3::ONE,
                        },
                        game_entity: GameEntity,
                    });
                    weapon_runtime.ammo -= 1;
                    weapon_runtime.state = WeaponState::Cooldown {
                        timer: weapon.fire_delay,
                    };

                    // if weapon_runtime.ammo > 0 {
                    // } else {
                    //     // weapon_runtime.state = WeaponState::Reloading {
                    //     //     timer: weapon.reload_time,
                    //     // };
                    // }
                }
            }
        }
    }
}

pub fn reload_weapon(
    mut messages: MessageReader<ReloadMessage>,
    children_query: Query<&Children, With<Actor>>,
    mut weapon_query: Query<(&Weapon, &mut WeaponRuntime), With<Weapon>>,
    audio_registry: Res<AudioRegistry>,
    mut commands: Commands,
) {
    for message in messages.read() {
        if let Ok(children) = children_query.get(message.entity) {
            for child in children.iter() {
                if let Ok((weapon, mut weapon_runtime)) = weapon_query.get_mut(child) {
                    if !matches!(weapon_runtime.state, WeaponState::Reloading { .. }) {
                        let Some(asset) = audio_registry.sounds.get(&weapon.reload_sound) else {
                            return;
                        };
                        commands
                            .spawn((AudioPlayer::new(asset.clone()), PlaybackSettings::DESPAWN));
                        weapon_runtime.state = WeaponState::Reloading {
                            timer: weapon.reload_time,
                        }
                    }
                }
            }
        }
    }
}

pub fn rotate_weapons(
    parent_query: Query<&ShootingIntent>,
    mut weapon_query: Query<(&ChildOf, &GlobalTransform, &mut Transform), With<Weapon>>,
) {
    for (parent, global_transform, mut transform) in weapon_query.iter_mut() {
        if let Ok(intent) = parent_query.get(parent.get()) {
            let direction =
                (intent.target_position - global_transform.translation().truncate()).normalize();

            let angle = direction.to_angle();
            transform.rotation = Quat::from_rotation_z(angle);
        }
    }
}

pub fn weapon_runtime_system(time: Res<Time>, mut query: Query<(&Weapon, &mut WeaponRuntime)>) {
    for (weapon_config, mut weapon_runtime) in query.iter_mut() {
        match weapon_runtime.state {
            WeaponState::Ready => {}
            WeaponState::Cooldown { mut timer } => {
                if timer > 0.0 {
                    timer -= time.delta_secs();
                    weapon_runtime.state = WeaponState::Cooldown { timer };
                } else {
                    weapon_runtime.state = WeaponState::Ready;
                };
            }
            WeaponState::Reloading { mut timer } => {
                if timer > 0.0 {
                    timer -= time.delta_secs();
                    weapon_runtime.state = WeaponState::Reloading { timer };
                } else {
                    weapon_runtime.ammo = weapon_config.magazine_size;
                    weapon_runtime.state = WeaponState::Ready;
                };
            }
        };
    }
}
