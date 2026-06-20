use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Component, Debug, Clone, Deserialize)]
pub struct Weapon {
    pub id: String,

    pub fire_delay: f32,
    pub reload_time: f32,
    pub magazine_size: u32,
    pub speed: f32,
    pub damage: f32,
    pub fire_mode: FireMode,

    // sounds
    pub dry_sound: String,
    pub shoot_sound: String,
    pub reload_sound: String,

    // spread
    pub spread_base: f32,
    pub spread_max: f32,
    pub spread_per_shot: f32,
    pub spread_recovery: f32,
    pub movement_spread: f32,
}

#[derive(Component, Debug)]
pub struct WeaponRuntime {
    pub state: WeaponState,
    pub ammo: u32,
    pub current_spread: f32,
}

impl WeaponRuntime {
    pub fn new_with_ammo(ammo: u32) -> Self {
        WeaponRuntime {
            state: WeaponState::Ready,
            ammo,
            current_spread: 0.,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum WeaponState {
    Ready,
    Cooldown { timer: f32 },
    Reloading { timer: f32 },
}

#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub enum FireMode {
    Semi,
    Auto,
}
