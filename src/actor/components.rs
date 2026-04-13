use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Component)]
pub struct Actor;

#[derive(Component, Debug, Serialize, Deserialize, Clone)]
pub struct Team(pub TeamId);

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum TeamId {
    Player,
    Zombie,
}

#[derive(Component, Debug)]
pub struct Zombie;
