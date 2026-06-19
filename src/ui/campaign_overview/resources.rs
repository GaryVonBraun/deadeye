use bevy::prelude::*;
use uuid::Uuid;

#[derive(Debug, Resource)]
pub struct SelectedMission {
    pub id: Option<Uuid>,
}
