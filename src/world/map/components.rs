use bevy::prelude::*;
use uuid::Uuid;

#[derive(Component, Debug)]
pub struct WorldMap {
    pub name: String,
    pub uuid: Uuid,
    pub tiles: Vec<Vec<u32>>,
}
