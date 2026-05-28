use bevy::prelude::*;

#[derive(Component, Debug)]
pub enum AiDirective {
    Idle,
    SearchAndDestroy,
    Follow(Entity),
}
