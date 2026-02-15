use bevy::prelude::*;
use crate::core::states::{AppState, SimulationState};

pub fn load_app(mut state: ResMut<NextState<AppState>>) {
    const INITIAL_STATE:AppState = AppState::MainMenu;
    
    info!("Finished loading, setting AppState to {:?}", INITIAL_STATE);
    state.set(INITIAL_STATE);
}

pub fn log_app_state_changes(
    mut transitions: MessageReader<StateTransitionEvent<AppState>>,
) {
    for event in transitions.read() {
        if let (Some(from), Some(to)) = (event.exited, event.entered) {
            info!("AppState changed: {:?} -> {:?}", from, to);
        }
    }
}