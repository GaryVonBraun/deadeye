use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*, window::WindowResolution};
use bevy_inspector_egui::bevy_egui::EguiPlugin;

use crate::{
    actor::ActorPlugin,
    ai::{AiPlugin, AiSet},
    collision::{CollisionPlugin, sets::PhysicsSet},
    combat::CombatPlugin,
    core::CorePlugin,
    debug::DebugPlugin,
    editor::EditorPlugin,
    map::MapPlugin,
    mission::MissionPlugin,
    navigation::NavigationPlugin,
    player::PlayerPlugin,
    props::PropsPlugin,
    ui::UiPlugin,
};
mod actor;
mod ai;
mod collision;
mod combat;
mod core;
mod debug;
mod editor;
mod map;
mod mission;
mod navigation;
mod player;
mod props;
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
        .add_plugins((EguiPlugin::default(), FrameTimeDiagnosticsPlugin::default()))
        // .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(CorePlugin)
        .add_plugins(bevy::diagnostic::LogDiagnosticsPlugin::default())
        .configure_sets(Update, AiSet::Targeting.before(PhysicsSet::Movement))
        .add_plugins((
            CollisionPlugin,
            ActorPlugin,
            PlayerPlugin,
            CombatPlugin,
            AiPlugin,
            MapPlugin,
            MissionPlugin,
            EditorPlugin,
            PropsPlugin,
            NavigationPlugin,
        ))
        .add_plugins(DebugPlugin)
        .add_plugins(UiPlugin)
        .run();
}
