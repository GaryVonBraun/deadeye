use bevy::prelude::*;
use uuid::Uuid;

#[derive(Component, Debug)]
pub struct MapMenuUi;

#[derive(Component, Debug)]
pub struct MapListUi;

#[derive(Component, Debug, Clone, Copy)]
pub enum MapListInteractions {
    Delete(Uuid),
    Edit,
}

#[derive(Component, Debug, Clone, Copy)]
pub enum MapMenuInteractions {
    Back,
    New,
}
