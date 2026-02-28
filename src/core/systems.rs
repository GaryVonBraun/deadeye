use crate::core::states::{AppState, SimulationState};
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
