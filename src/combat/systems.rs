use bevy::prelude::*;

use crate::{
    actor::components::Actor,
    collision::systems::check_collision,
    combat::{
        components::{MeleeIntent, MeleeState},
        health::{
            components::{Hitbox, Hurtbox},
            messages::DamageMessage,
        },
    },
};

pub fn tick_melee_intents(
    mut intent_query: Query<(&mut MeleeIntent, &Transform, &Hitbox), With<Actor>>,
    actors: Query<(&Transform, &Hurtbox), With<Actor>>,
    time: Res<Time>,
    mut damage_writer: MessageWriter<DamageMessage>,
) {
    for (mut melee_intent, transform, hitbox) in intent_query.iter_mut() {
        match melee_intent.melee_state {
            MeleeState::Ready => continue,
            MeleeState::AttackDelay(delay) => {
                if delay <= 0. {
                    melee_intent.melee_state = MeleeState::Cooldown(melee_intent.cooldown);
                    let Some(target) = melee_intent.target else {
                        error!("No target entity found for attack");
                        continue;
                    };

                    let Ok((target_transform, target_hurtbox)) = actors.get(target) else {
                        warn!("Failed to find target entity");
                        continue;
                    };

                    if check_collision(
                        transform.translation.truncate(),
                        hitbox,
                        target_transform.translation.truncate(),
                        target_hurtbox,
                    ) {
                        damage_writer.write(DamageMessage {
                            target,
                            amount: melee_intent.damage,
                        });
                        info!("successful attack");
                        continue;
                    }

                    info!("missed attack, out of range");
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
