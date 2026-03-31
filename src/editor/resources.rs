use bevy::prelude::*;
#[derive(Debug, Resource)]
pub struct ActiveTile {
    pub index: u16,
}

#[derive(Debug, Resource, Default)]
pub enum EditorTool {
    #[default]
    TilePainter,
    Prop,
    PlayerSpawn,
}
