use crate::{
    core::{
        components::GameEntity,
        states::{AppState, SimulationState},
    },
    map::{rendering::resources::TilesetRenderState, resources::ActiveMap},
    mission::resources::ActiveMission,
    navigation::resources::NavGrid,
};
use bevy::prelude::*;

pub fn load_app(mut state: ResMut<NextState<AppState>>) {
    const INITIAL_STATE: AppState = AppState::MainMenu;

    info!("Finished loading, setting AppState to {:?}", INITIAL_STATE);
    state.set(INITIAL_STATE);
}

pub fn log_app_state_changes(mut transitions: MessageReader<StateTransitionEvent<AppState>>) {
    for event in transitions.read() {
        if let (Some(from), Some(to)) = (event.exited, event.entered) {
            info!("AppState changed: {:?} -> {:?}", from, to);
        }
    }
}
pub fn log_simulation_state_changes(
    mut transitions: MessageReader<StateTransitionEvent<SimulationState>>,
) {
    for event in transitions.read() {
        if let (Some(from), Some(to)) = (event.exited, event.entered) {
            info!("AppState changed: {:?} -> {:?}", from, to);
        }
    }
}

pub fn toggle_simulation_state(
    keys: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<SimulationState>>,
    state: Res<State<SimulationState>>,
) {
    if keys.just_pressed(KeyCode::KeyP) {
        match state.get() {
            SimulationState::Running => next_state.set(SimulationState::Paused),
            SimulationState::Paused => next_state.set(SimulationState::Running),
        }
    }
}

pub fn toggle_app_state(
    keys: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<AppState>>,
    state: Res<State<AppState>>,
) {
    if keys.just_pressed(KeyCode::KeyO) {
        match state.get() {
            AppState::MainMenu => next_state.set(AppState::InGame),
            AppState::InGame => next_state.set(AppState::MainMenu),
            _ => {}
        }
    }
}

pub fn run_simulation(mut next_state: ResMut<NextState<SimulationState>>) {
    next_state.set(SimulationState::Running);
}

pub fn despawn_game_entities(query: Query<Entity, With<GameEntity>>, mut commands: Commands) {
    info!("Despawning GameEntity amount: {}", query.iter().count());
    for entity in query.iter() {
        // info!("despawning: {:?}", entity);
        commands.entity(entity).despawn();
    }
}

pub fn remove_resources(mut commands: Commands) {
    commands.remove_resource::<ActiveMap>();
    commands.remove_resource::<ActiveMission>();
    commands.remove_resource::<NavGrid>();
    commands.remove_resource::<TilesetRenderState>();
}

//NOTE - Idk what this is/was for
pub fn world_to_hash(world_pos: Vec2, cell_size: f32) -> (i32, i32) {
    let cell_x = (world_pos.x / cell_size).floor() as i32;
    let cell_y = (world_pos.y / cell_size).floor() as i32;
    return (cell_x, cell_y);
}
