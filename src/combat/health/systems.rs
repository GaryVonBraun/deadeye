use bevy::prelude::*;

use crate::combat::health::{components::Health, messages::DamageMessage};

pub fn apply_damage(
    mut messages: MessageReader<DamageMessage>,
    mut health_query: Query<&mut Health>,
    mut commands: Commands,
) {
    for message in messages.read() {
        let Ok(mut health) = health_query.get_mut(message.target) else {
            break;
        };

        health.current -= message.amount;
        info!(
            "entity {:?} took {:?} damage, current health is {:?}",
            message.target, message.amount, health.current
        );

        //TEMPORARY - we just "kill" the entity right here if below 0 health
        if health.current <= 0.0 {
            commands.entity(message.target).despawn();
            info!(
                "[TEMPORARY] - entity {:?} despawned health below 0",
                message.target
            );
        }
    }
}
