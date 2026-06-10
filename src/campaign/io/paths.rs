use std::path::PathBuf;

use uuid::Uuid;

pub fn campaign_data_path(id: &Uuid) -> PathBuf {
    PathBuf::from(format!("content/campaigns/{}.ron", id.to_string()))
}
