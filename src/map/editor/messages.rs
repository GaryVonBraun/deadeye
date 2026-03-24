use bevy::prelude::*;

#[derive(Debug, Message, Clone)]
pub struct UpdateMapBoundsMessage {
    pub direction: MapBoundDirectionEnum,
    pub action: MapBoundOperationEnum,
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
