use bevy::prelude::*;

pub mod components;
pub mod io;

pub struct PropsPlugin;

impl Plugin for PropsPlugin {
    fn build(&self, app: &mut App) {}
}
