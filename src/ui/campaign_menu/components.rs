use bevy::prelude::*;
use uuid::Uuid;

#[derive(Component, Debug)]
pub struct MainMenu;

#[derive(Component, Debug, Clone, Copy)]
pub enum CampaignMenuInteractions {
    BackButton,
    LoadButton(Uuid),
}
