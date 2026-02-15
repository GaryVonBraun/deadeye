use bevy::prelude::*;

#[derive(Component)]
pub struct HumanoidAppearance;

#[derive(Bundle)]
pub struct HumanoidAppearanceBundle{
    pub sprite: Sprite,
    pub appearance:HumanoidAppearance
}