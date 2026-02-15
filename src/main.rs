use bevy::{prelude::*, window::WindowResolution};

use crate::{
    actor::ActorPlugin, core::CorePlugin, debug::DebugPlugin, player::PlayerPlugin,
    simulation::SimulationPlugin,
};
mod actor;
mod core;
mod debug;
mod player;
mod simulation;

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
        .add_plugins((SimulationPlugin, ActorPlugin, PlayerPlugin))
        .add_plugins(CorePlugin)
        .add_plugins(DebugPlugin)
        .run();
}
