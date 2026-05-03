use bevy::prelude::*;

use crate::{
    core::io::read_ron_file,
    props::io::{paths::prop_definition_path, types::PropDefinitions},
};

pub fn read_prop_definitions() -> Result<PropDefinitions, ()> {
    match read_ron_file(prop_definition_path()) {
        Ok(definitions) => Ok(definitions),
        Err(()) => {
            error!("Invalid Prop definition");
            Err(())
        }
    }
}
