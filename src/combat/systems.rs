use bevy::prelude::*;

use crate::{
    actor::components::Actor,
    combat::components::{MeleeIntent, MeleeState},
};

pub fn melee_attack_handler(
    mut intent_query: Query<&mut MeleeIntent, With<Actor>>,
    time: Res<Time>,
) {
    for mut melee_intent in intent_query.iter_mut() {
        match melee_intent.melee_state {
            MeleeState::Ready => continue,
            MeleeState::AttackDelay(delay) => {
                if delay <= 0. {
                    info!("entity attacked");
                    melee_intent.melee_state = MeleeState::AttackDelay(melee_intent.cooldown);
                    continue;
                }
                melee_intent.melee_state = MeleeState::AttackDelay(delay - time.delta_secs());
            }
            MeleeState::Cooldown(cooldown) => {
                if cooldown <= 0. {
                    melee_intent.melee_state = MeleeState::Ready;
                    info!("entity ready to attack again");
                    continue;
                }
                melee_intent.melee_state = MeleeState::Cooldown(cooldown - time.delta_secs());
            }
        }
    }
}
