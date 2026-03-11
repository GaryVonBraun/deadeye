use bevy::prelude::*;
use std::{fs, path::PathBuf};

use serde::{Serialize, de::DeserializeOwned};

pub fn read_ron_file<T>(path: PathBuf) -> Result<T, String>
where
    T: DeserializeOwned,
{
    if path.exists() {
        let contents = fs::read_to_string(path);
        match contents {
            Ok(contents) => match ron::from_str::<T>(&contents) {
                Ok(manifest) => Ok(manifest),
                Err(spanned_error) => {
                    error!("{:?}", spanned_error.code);
                    Err(spanned_error.code.to_string())
                }
            },
            Err(_) => {
                error!("error parsing content");
                Err("error parsing content".to_string())
            }
        }
    } else {
        error!("invalid path: {:?}", path);
        Err(format!("invalid path: {:?}", path).to_string())
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
