use bevy::prelude::*;

use crate::{
    actor::components::Actor,
    combat::{
        components::{MeleeIntent, MeleeState},
        health::messages::DamageMessage,
    },
    simulation::collision::components::Collision,
};

pub fn melee_attack_handler(
    mut intent_query: Query<(&mut MeleeIntent, &Transform, &Collision), With<Actor>>,
    actors: Query<(&Transform, &Collision), With<Actor>>,
    time: Res<Time>,
    mut damage_writer: MessageWriter<DamageMessage>,
) {
    for (mut melee_intent, transform, collision) in intent_query.iter_mut() {
        match melee_intent.melee_state {
            MeleeState::Ready => continue,
            MeleeState::AttackDelay(delay) => {
                if delay <= 0. {
                    melee_intent.melee_state = MeleeState::Cooldown(melee_intent.cooldown);
                    let Some(target) = melee_intent.target else {
                        error!("No target entity found for attack");
                        continue;
                    };

                    let Ok((actor_transform, actor_collision)) = actors.get(target) else {
                        warn!("Failed to find target entity");
                        continue;
                    };

                    if Vec2::distance(
                        transform.translation.truncate(),
                        actor_transform.translation.truncate(),
                    ) <= melee_intent.range + collision.radius + actor_collision.radius
                    {
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
