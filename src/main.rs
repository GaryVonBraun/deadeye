use bevy::{prelude::*, window::WindowResolution};

use crate::{
    actor::ActorPlugin, ai::AiPlugin, combat::CombatPlugin, core::CorePlugin, debug::DebugPlugin,
    player::PlayerPlugin, simulation::SimulationPlugin, ui::UiPlugin,
};
mod actor;
mod ai;
mod combat;
mod core;
mod debug;
mod player;
mod simulation;
mod ui;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: String::from("Deadeye"),
                position: WindowPosition::Centered(MonitorSelection::Primary),
                resolution: WindowResolution::new(2000, 1000),
                resizable: true,
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_plugins(CorePlugin)
        .add_plugins((
            SimulationPlugin,
            ActorPlugin,
            PlayerPlugin,
            CombatPlugin,
            AiPlugin,
        ))
        .add_plugins(DebugPlugin)
        .add_plugins(UiPlugin)
        .run();
}
// brb