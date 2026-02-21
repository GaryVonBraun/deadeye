use bevy::prelude::*;

use crate::{
    actor::components::Actor,
    combat::{messages::ShootMessage, weapon::component::Weapon},
};

pub fn shoot_weapon(
    mut messages: MessageReader<ShootMessage>,
    children_query: Query<&Children, With<Actor>>,
    weapon_query: Query<&mut Weapon, With<Weapon>>,
) {
    for message in messages.read() {
        // get the children of the shooter entity
        if let Ok(children) = children_query.get(message.shooter) {
            for child in children.iter() {
                // and we check if the child is a weapon
                if let Ok(mut weapon) = weapon_query.get(child) {
                    //NOTE - currently this means if an actor has multiple weapons they all fire
                    info!(
                        "weapon shot with a projectile speed of {:?}",
                        weapon.projectile_speed
                    )
                }
            }
        }
    }
}
