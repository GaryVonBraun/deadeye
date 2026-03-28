use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Chunk {
    pub grid: Vec<Vec<u32>>,
    pub chunk_pos: IVec2,
}
