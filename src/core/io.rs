use bevy::prelude::*;
use std::{fs, path::PathBuf};

use serde::{Serialize, de::DeserializeOwned};

pub fn read_ron_file<T>(path: PathBuf) -> Result<T, ()>
where
    T: DeserializeOwned,
{
    // if no path found return error
    if !path.exists() {
        error!("invalid path: {:?}", path);
        return Err(());
    }

    // if there is no content return error
    let Ok(contents) = fs::read_to_string(path.clone()) else {
        error!("failed to read content of: {:?}", path);
        return Err(());
    };

    match ron::from_str::<T>(&contents) {
        Ok(manifest) => Ok(manifest),
        Err(spanned_error) => {
            error!("{:?}", spanned_error.code);
            Err(())
        }
    }
}
pub fn write_ron_file<T>(data: &T, path: PathBuf) -> Result<(), String>
where
    T: Serialize,
{
    match ron::to_string(data) {
        Ok(parsed_data) => match fs::write(path, parsed_data) {
            Ok(result) => Ok(result),
            Err(err) => {
                error!("{:?}", err);
                Err(err.to_string())
            }
        },
        Err(err) => {
            error!("{:?}", err);
            Err(err.to_string())
        }
    }
}

pub fn remove_ron_file(path: PathBuf) -> Result<(), ()> {
    // if no path found return error
    if !path.exists() {
        error!("invalid path: {:?}", path);
        return Err(());
    }

    match fs::remove_file(path) {
        Ok(result) => Ok(result),
        Err(err) => {
            error!("{:?}", err);
            Err(())
        }
    }
}
