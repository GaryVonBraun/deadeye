use bevy::prelude::*;

#[derive(Debug, Resource)]
pub struct NavGrid {
    pub cells: Vec<Vec<bool>>,
    pub width: u32,
    pub height: u32,
}
