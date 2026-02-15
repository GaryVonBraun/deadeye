use bevy::{prelude::*, window::WindowResolution};

use crate::{actor::ActorPlugin, core::CorePlugin, debug::DebugPlugin, simulation::SimulationPlugin};
mod core;
mod simulation;
mod actor;
mod debug;
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
        .add_plugins((SimulationPlugin, ActorPlugin))
        .add_plugins(CorePlugin)
        .add_plugins(DebugPlugin)
        .run();
}
