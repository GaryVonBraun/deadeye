use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct MainMenu;

#[derive(Component, Debug, Clone, Copy)]
pub enum CampaignMenuInteractions {
    BackButton,
}
