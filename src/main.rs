use bevy::{prelude::*, window::WindowResolution};
use bevy_egui::EguiPrimaryContextPass;
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};

use crate::{
    actor::ActorPlugin, ai::AiPlugin, combat::CombatPlugin, core::CorePlugin, debug::DebugPlugin,
    map::MapPlugin, mission::MissionPlugin, player::PlayerPlugin, simulation::SimulationPlugin,
    ui::UiPlugin,
};
mod actor;
mod ai;
mod combat;
mod core;
mod debug;
mod map;
mod mission;
mod player;
mod simulation;
mod ui;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: String::from("Deadeye"),
                        position: WindowPosition::Centered(MonitorSelection::Primary),
                        resolution: WindowResolution::new(2000, 1000),
                        resizable: true,
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugins(EguiPlugin::default())
        // .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(CorePlugin)
        .add_plugins((
            SimulationPlugin,
            ActorPlugin,
            PlayerPlugin,
            CombatPlugin,
            AiPlugin,
            MapPlugin,
            MissionPlugin,
        ))
        .add_plugins(DebugPlugin)
        .add_plugins(UiPlugin)
        .run();
}
