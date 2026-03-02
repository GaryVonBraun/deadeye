use bevy::prelude::*;

use crate::combat::health::{
    components::{Dead, Health},
    messages::DamageMessage,
};

pub fn apply_damage(
    mut messages: MessageReader<DamageMessage>,
    mut health_query: Query<&mut Health>,
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
    }
}

pub fn death_system(mut commands: Commands, query: Query<(Entity, &Health), Without<Dead>>) {
    for (entity, health) in query.iter() {
        if health.current <= 0.0 {
            //TEMPORARY - For now we just despawn the entity if it dies
            // commands.entity(entity).insert(Dead);
            commands.entity(entity).despawn();
        }
    }
}
