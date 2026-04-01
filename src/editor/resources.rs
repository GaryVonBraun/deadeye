use bevy::prelude::*;

#[derive(Debug, Resource, Default)]
pub enum EditorTool {
    #[default]
    TilePainter,
    PropTool(ToolAction),
    PlayerSpawn,
}

#[derive(Debug, Resource)]
pub struct ActiveTile {
    pub index: u16,
}

#[derive(Debug, Resource)]
pub struct SelectedProp {
    pub name: String,
}

#[derive(Debug, Clone, Copy)]
pub enum ToolAction {
    Place,
}
