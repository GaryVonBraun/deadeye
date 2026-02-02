use bevy::{prelude::*, window::WindowResolution};

use crate::{core::CorePlugin, gameplay::GameplayPlugin};
mod core;
mod gameplay;
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
        .add_plugins(GameplayPlugin)
        .add_plugins(CorePlugin)
        .run();
}
