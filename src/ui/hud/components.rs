use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Hud;

#[derive(Component, Debug, Clone, Copy)]
pub enum VictoryMenuInteractions {
    MissionsButton,
    QuitButton,
}

#[derive(Component, Debug, Clone, Copy)]
pub struct HudHealthBar {
    pub value: f32,
}

#[derive(Component, Debug)]
pub struct HudZombieCount {
    pub value: usize,
}

#[derive(Component, Debug)]
pub struct HudWaves {
    pub current_wave: usize,
}
