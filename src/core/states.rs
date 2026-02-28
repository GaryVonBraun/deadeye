use bevy::prelude::*;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default, Copy)]
pub enum AppState {
    MainMenu,
    // Settings,
    InGame,
    // GameOver,
    #[default]
    Loading,
}

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default, Copy)]
pub enum SimulationState {
    Running,
    #[default]
    Paused,
}
