use bevy::prelude::*;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default, Copy)]
pub enum AppState {
    MainMenu,
    MapListMenu,
    // Settings,
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
