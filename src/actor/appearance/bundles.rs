use bevy::prelude::*;

#[derive(Component)]
pub struct Appearance;

#[derive(Bundle)]
pub struct AppearanceBundle{
    pub sprite: Sprite,
    pub appearance:Appearance
}