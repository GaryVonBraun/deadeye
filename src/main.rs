use bevy::{prelude::*, window::WindowResolution};
use bevy_egui::EguiPrimaryContextPass;
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};

use crate::{
    actor::ActorPlugin, ai::AiPlugin, combat::CombatPlugin, core::CorePlugin, debug::DebugPlugin,
    mission::MissionPlugin, player::PlayerPlugin, simulation::SimulationPlugin, ui::UiPlugin,
    world::WorldPlugin,
};
mod actor;
mod ai;
mod combat;
mod core;
mod debug;
mod mission;
mod player;
mod simulation;
mod ui;
mod world;

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
        .add_plugins(EguiPlugin::default())
        // .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(CorePlugin)
        .add_plugins((
            SimulationPlugin,
            ActorPlugin,
            PlayerPlugin,
            CombatPlugin,
            AiPlugin,
            WorldPlugin,
            MissionPlugin,
        ))
        .add_plugins(DebugPlugin)
        .add_plugins(UiPlugin)
        .run();
}
