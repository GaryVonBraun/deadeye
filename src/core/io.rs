use std::{fs, path::PathBuf};

use bevy::tasks::futures_lite::io;
use serde::{Serialize, Serializer, de::DeserializeOwned};

pub fn read_ron_file<T>(path: PathBuf) -> Result<T, String>
where
    T: DeserializeOwned,
{
    if path.exists() {
        let contents = fs::read_to_string(path);
        match contents {
            Ok(contents) => match ron::from_str::<T>(&contents) {
                Ok(manifest) => Ok(manifest),
                Err(spanned_error) => Err(spanned_error.code.to_string()),
            },
            Err(_) => Err("error parsing content".to_string()),
        }
    } else {
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
            Err(err) => Err(err.to_string()),
        },
        Err(err) => Err(err.to_string()),
    }
}
