use bevy::prelude::*;

use crate::{
    collision::components::Collision, core::components::GameEntity, props::components::Prop,
};

#[derive(Bundle)]
pub struct PropBundle {
    pub prop: Prop,
    pub sprite: Sprite,
    pub transform: Transform,
    pub collision: Collision,
    pub game_entity: GameEntity,
}
