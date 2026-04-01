use bevy::prelude::*;

#[derive(Debug, Resource, Default, Clone)]
pub enum EditorTools {
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
