use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct GameOverMenu;

#[derive(Component, Debug, Clone, Copy)]
pub enum GameOverMenuInteractions {
    RetryButton,
    MissionsButton,
    QuitButton,
}
