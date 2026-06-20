use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::combat::weapon::components::Weapon;

#[derive(Debug, Deserialize)]
pub struct WeaponDefinitions {
    pub weapons: Vec<Weapon>,
}
