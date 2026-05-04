use bevy::prelude::*;
use uuid::Uuid;

#[derive(Debug, Message, Clone)]
pub struct UpdateMapBoundsMessage {
    pub direction: MapBoundDirectionEnum,
    pub action: MapBoundOperationEnum,
    pub grow_tile: u32,
}

#[derive(Debug, Clone)]
pub enum MapBoundDirectionEnum {
    North,
    East,
    South,
    West,
}

#[derive(Debug, Clone)]
pub enum MapBoundOperationEnum {
    Expand(u32),
    Shrink(u32),
}

#[derive(Message)]
pub struct LoadEditorMessage {
    pub id: Uuid,
}

#[derive(Debug, Message)]
pub struct SaveEditorChangesMessage;
