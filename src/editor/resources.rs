use bevy::prelude::*;
use bevy_egui::egui;

#[derive(Debug, Resource, Default, Clone)]
pub enum EditorTool {
    #[default]
    None,
    TilePainter {
        x: u32,
        y: u32,
    },
    PropTool(ToolAction),
    PlayerSpawn,
}

#[derive(Debug, Clone)]
pub enum ToolAction {
    Place(String),
    Remove,
}

#[derive(Debug, Resource)]
pub struct EditorSettings {
    pub snap_to_grid: bool,
    pub tile_aligned: bool,
    pub size_control_amount: u32,
    pub tile_picker_zoom: f32,
    pub tile_picker_offset: egui::Vec2,
    pub grow_tile: u32,
}
