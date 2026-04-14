use bevy::prelude::*;
use uuid::Uuid;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default, Copy)]
pub enum AppState {
    MainMenu,
    MissionMenu,
    Editor,
    InGame,
    GameOver,
    #[default]
    Loading,
}

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default, Copy)]
pub enum SimulationState {
    Running,
    #[default]
    Paused,
}
