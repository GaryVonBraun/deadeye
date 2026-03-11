use std::{fs, path::PathBuf};

use serde::de::DeserializeOwned;

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
        Err("invalid path".to_string())
    }
}
