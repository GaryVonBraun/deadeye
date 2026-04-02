use bevy::prelude::*;

#[derive(Debug, Resource, Default, Clone)]
pub enum EditorTool {
    #[default]
    None,
    TilePainter(u16),
    PropTool(ToolAction),
    PlayerSpawn,
}

#[derive(Debug, Clone)]
pub enum ToolAction {
    Place(String),
}

#[derive(Debug, Resource)]
pub struct EditorSettings {
    pub snap_to_grid: bool,
}
