use bevy::prelude::*;
use uuid::Uuid;

#[derive(Component, Debug)]
pub struct MissionDevMenuUi;

#[derive(Component, Debug)]
pub struct MissionListUi;

#[derive(Component, Debug, Clone, Copy)]
pub enum MissionListInteractions {
    Delete { mission_id: Uuid, map_id: Uuid },
    Edit(Uuid),
}

#[derive(Component, Debug, Clone, Copy)]
pub enum MissionMenuInteractions {
    Back,
    New,
}
