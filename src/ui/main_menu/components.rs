use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct MainMenu;

#[derive(Component, Debug, Clone, Copy)]
pub enum MainMenuInteractions {
    ContinueButton,
    LoadButton,
    SettingsButton,
    MissionsButton,
    QuitButton,
}
