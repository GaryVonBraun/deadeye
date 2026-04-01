use bevy::prelude::*;

use crate::props::io::types::PropDefinition;
#[derive(Debug, Resource)]
pub struct ActiveMapProps {
    props: Vec<PropDefinition>,
}
