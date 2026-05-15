use bevy::prelude::*;

#[derive(Component, Debug, Clone)]
pub struct Weapon {
    pub fire_delay: f32,
    pub reload_time: f32,
    pub magazine_size: u32,
    pub speed: f32,
    pub damage: f32,
    pub shoot_sound: String,
    pub reload_sound: String,
    pub dry_sound: String,
}

#[derive(Component, Debug)]
pub struct WeaponRuntime {
    pub state: WeaponState,
    pub ammo: u32,
}

#[derive(Debug, PartialEq)]
pub enum WeaponState {
    Ready,
    Cooldown { timer: f32 },
    Reloading { timer: f32 },
}
