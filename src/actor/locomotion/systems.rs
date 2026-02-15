use bevy::prelude::*;

use crate::actor::locomotion::components::Locomotion;

pub fn apply_locomotion(mut query: Query<(&Locomotion, &mut Transform)>, time: Res<Time>) {
    for (locomotion, mut transform) in &mut query {
        let direction = locomotion.movement_intent.normalize_or_zero();

        let movement = direction * locomotion.movement_speed * time.delta_secs();

        transform.translation += movement.extend(0.0);
    }
}
