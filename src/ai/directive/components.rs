use bevy::prelude::*;

#[derive(Component, Debug, PartialEq, Clone)]
pub enum AiDirective {
    Idle,
    SearchAndDestroy,
    Follow(Entity),
}
