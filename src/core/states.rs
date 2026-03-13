use bevy::prelude::*;
use uuid::Uuid;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default, Copy)]
pub enum AppState {
    MainMenu,
    MapListMenu,
    MapEditor,
    InGame,
    #[default]
    Loading,
}

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default, Copy)]
pub enum SimulationState {
    Running,
    #[default]
    Paused,
}
