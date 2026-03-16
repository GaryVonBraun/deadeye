use bevy::prelude::*;
use uuid::Uuid;

#[derive(Component, Debug)]
pub struct MissionDevMenuUi;

#[derive(Component, Debug)]
pub struct MissionDevListUi;

#[derive(Component, Debug, Clone, Copy)]
pub enum MissionDevListInteractions {
    Delete(Uuid),
    Edit(Uuid),
}

#[derive(Component, Debug, Clone, Copy)]
pub enum MissionDevMenuInteractions {
    Back,
    New,
}
