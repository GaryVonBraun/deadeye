use bevy::{platform::collections::HashMap, prelude::*};

use crate::{collision::components::Collision, props::io::types::PlacedProp};
#[derive(Debug, Resource)]
pub struct ActiveMapProps {
    pub props: Vec<PlacedProp>,
}

#[derive(Debug, Resource)]
pub struct PropSpatialHash {
    pub grid: HashMap<(i32, i32), Vec<(Vec2, Collision)>>,
    pub cell_size: f32,
}
