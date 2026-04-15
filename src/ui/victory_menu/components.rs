use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct VictoryMenu;

#[derive(Component, Debug, Clone, Copy)]
pub enum VictoryMenuInteractions {
    MissionsButton,
    QuitButton,
}
