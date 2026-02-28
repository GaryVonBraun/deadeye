use crate::{
    core::{
        components::GameEntity,
        states::{AppState, SimulationState},
    },
    player::components::Player,
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
            AppState::Loading => todo!(),
        }
    }
}

pub fn run_simulation(mut next_state: ResMut<NextState<SimulationState>>) {
    next_state.set(SimulationState::Running);
}

pub fn despawn_game_entities(query: Query<Entity, With<GameEntity>>, mut commands: Commands) {
    info!("GameEntity count: {}", query.iter().count());
    for entity in query.iter() {
        info!("despawning: {:?}", entity);
        commands.entity(entity).despawn();
    }
}

pub fn camera_follow(
    mut camera_query: Query<&mut Transform, (With<Camera>, Without<Player>)>,
    player_query: Query<&Transform, With<Player>>,
) {
    let Ok(mut camera_transform) = camera_query.single_mut() else {
        error!("no camera found");
        return;
    };

    let Ok(player_transform) = player_query.single() else {
        return;
    };
    camera_transform.translation = player_transform.translation;
}
