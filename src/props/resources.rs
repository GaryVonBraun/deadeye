use bevy::prelude::*;

use crate::props::io::types::PlacedProp;
#[derive(Debug, Resource)]
pub struct ActiveMapProps {
    pub props: Vec<PlacedProp>,
}
