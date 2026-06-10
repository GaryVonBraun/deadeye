use bevy::prelude::*;
use uuid::Uuid;

#[derive(Debug, Message)]
pub struct CreateCampaignMessage;

#[derive(Debug, Message)]
pub struct LoadCampaignMessage {
    pub id: Uuid,
}
