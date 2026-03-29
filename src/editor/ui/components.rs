use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct MissionEditorUi;

#[derive(Component, Debug, Clone, Copy)]
pub enum MissionEditorInteractions {
    Back,
}
