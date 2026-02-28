use bevy::{prelude::*, window::WindowResolution};

use crate::{
    actor::ActorPlugin, combat::CombatPlugin, core::CorePlugin, debug::DebugPlugin,
    player::PlayerPlugin, simulation::SimulationPlugin, ui::UiPlugin,
};
mod actor;
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
                resolution: WindowResolution::new(512, 512),
                resizable: false,
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_plugins((SimulationPlugin, ActorPlugin, PlayerPlugin, CombatPlugin))
        .add_plugins(CorePlugin)
        .add_plugins(DebugPlugin)
        .add_plugins(UiPlugin)
        .run();
}
