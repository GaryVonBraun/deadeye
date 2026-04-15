use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Mission {
    pub id: Uuid,
    pub name: String,
    pub map_id: Uuid,
    pub player_spawn: Vec2,
    pub spawn_inset: u32,
    pub waves: Vec<WaveDefinition>,
}

#[derive(Debug, Resource)]
pub struct ActiveMission {
    pub mission: Mission,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WaveDefinition {
    pub zombie_count: u32,
    pub spawn_per_second: f32,
    pub delay_seconds: f32,
}

impl WaveDefinition {
    pub fn default() -> Self {
        WaveDefinition {
            zombie_count: 0,
            spawn_per_second: 0.,
            delay_seconds: 0.,
        }
    }
}

#[derive(Debug, Resource)]
pub struct WaveSpawnerState {
    pub current_wave: usize,
    pub wave_zombies_spawned: u32,
    pub spawn_timer: Timer,
    pub wave_delay_timer: Option<Timer>,
    pub finished: bool,
}
