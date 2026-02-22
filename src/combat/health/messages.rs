use bevy::prelude::*;

#[derive(Message)]
pub struct DamageMessage {
    pub target: Entity,
    pub amount: f32,
}
