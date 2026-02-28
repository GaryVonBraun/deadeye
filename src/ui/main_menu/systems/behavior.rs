use bevy::prelude::*;

use crate::{core::states::AppState, ui::main_menu::components::MainMenuInteractions};

pub fn main_menu_interactions(
    mut button_query: Query<
        (&Interaction, &MainMenuInteractions),
        (Changed<Interaction>, With<MainMenuInteractions>),
    >,
    mut next_state: ResMut<NextState<AppState>>,
    mut app_exit_message_writer: MessageWriter<AppExit>,
) {
    for (interaction, &menu_interaction) in button_query.iter_mut() {
        if *interaction == Interaction::Pressed {
            match menu_interaction {
                MainMenuInteractions::PlayButton => {
                    next_state.set(AppState::InGame);
                }
                MainMenuInteractions::SettingsButton => {
                    //TEMPORARY - currently settings don't exist so its placeholder
                    next_state.set(AppState::InGame);
                }
                MainMenuInteractions::QuitButton => {
                    app_exit_message_writer.write(AppExit::Success);
                }
            }
        }
    }
}
