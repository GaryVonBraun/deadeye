use bevy::prelude::*;

use crate::{
    combat::weapon::io::{paths::weapon_definition_path, types::WeaponDefinitions},
    core::io::read_ron_file,
};

pub fn read_weapon_definitions() -> Result<WeaponDefinitions, ()> {
    match read_ron_file(weapon_definition_path()) {
        Ok(definitions) => Ok(definitions),
        Err(()) => {
            error!("Invalid Prop definition");
            Err(())
        }
    }
}
