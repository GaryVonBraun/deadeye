use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct ShootingIntent {
    pub direction: Vec2,
}

impl ShootingIntent {
    pub fn default() -> Self {
        ShootingIntent {
            direction: Vec2::default(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum MeleeState {
    Ready,
    AttackDelay(f32),
    Cooldown(f32),
}

#[derive(Component, Debug)]
pub struct MeleeIntent {
    pub target: Option<Entity>,
    pub melee_state: MeleeState,
    pub delay: f32,
    pub cooldown: f32,
    pub range: f32,
    pub damage: f32,
}

#[derive(Component)]
pub struct EquippedWeapon {
    pub entity: Entity,
}
