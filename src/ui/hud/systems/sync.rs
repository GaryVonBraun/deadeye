use bevy::prelude::*;

use crate::{
    actor::components::Zombie,
    combat::{
        components::EquippedWeapon,
        health::components::Health,
        weapon::components::{Weapon, WeaponRuntime},
    },
    mission::resources::WaveSpawnerState,
    player::components::Player,
    ui::hud::components::{HudHealthBar, HudWaves, HudWeaponInfo, HudZombieCount},
};

pub fn update_health_bar(
    player_query: Query<&Health, With<Player>>,
    mut hud_query: Query<(&mut Node, &mut HudHealthBar)>,
) {
    let Ok(player_health) = player_query.single() else {
        return;
    };

    let Ok((mut node, mut hud_health)) = hud_query.single_mut() else {
        return;
    };

    if hud_health.value == player_health.current {
        return;
    }
    hud_health.value = player_health.current;

    node.width = Val::Percent((player_health.current / player_health.max) * (100.));
}

pub fn update_zombie_count(
    zombie_query: Query<Entity, With<Zombie>>,
    mut hud_query: Query<(&mut Text, &mut HudZombieCount)>,
) {
    let Ok((mut text, mut zombie_count)) = hud_query.single_mut() else {
        return;
    };

    if zombie_query.count() != zombie_count.value {
        zombie_count.value = zombie_query.count();

        text.0 = format!("Zombie amount: {}", zombie_count.value);
    }
}

pub fn update_waves(
    state: Res<WaveSpawnerState>,
    mut hud_query: Query<(&mut Text, &mut HudWaves)>,
) {
    let Ok((mut text, mut hud_waves)) = hud_query.single_mut() else {
        return;
    };

    if state.current_wave != hud_waves.current_wave {
        hud_waves.current_wave = state.current_wave;

        text.0 = format!("Wave: {}", hud_waves.current_wave);
    }
}

pub fn update_weapon_information(
    mut hud_query: Query<(&mut Text, &mut HudWeaponInfo)>,
    weapon_query: Query<(&Weapon, &mut WeaponRuntime), With<Weapon>>,
    player_query: Query<&EquippedWeapon, With<Player>>,
) {
    let Ok((mut text, mut hud_weapon_info)) = hud_query.single_mut() else {
        return;
    };
    let Ok(equipped_weapon) = player_query.single() else {
        return;
    };

    let Ok((weapon, weapon_runtime)) = weapon_query.get(equipped_weapon.entity) else {
        return;
    };
    text.0 = format!("Ammo: {}", weapon_runtime.ammo);
}
