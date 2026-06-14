use bevy::prelude::*;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default, Copy)]
pub enum AppState {
    MainMenu,
    CampaignMenu,
    CampaignOverview,
    MissionMenu,
    Editor,
    InGame,
    GameOver,
    Victory,
    #[default]
    Loading,
}

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default, Copy)]
pub enum SimulationState {
    Running,
    #[default]
    Paused,
}
