use bevy::prelude::*;

use crate::{
    core::states::AppState,
    mission::{messages::LoadMissionMessage, resources::GameOverData},
    ui::game_over_menu::components::GameOverMenuInteractions,
};

pub fn game_over_menu_interactions(
    mut button_query: Query<
        (&Interaction, &GameOverMenuInteractions),
        (Changed<Interaction>, With<GameOverMenuInteractions>),
    >,
    mut next_state: ResMut<NextState<AppState>>,
    mut app_exit_message_writer: MessageWriter<AppExit>,
    mut load_mission_writer: MessageWriter<LoadMissionMessage>,
    game_over_data: Res<GameOverData>,
) {
    for (interaction, &menu_interaction) in button_query.iter_mut() {
        if *interaction == Interaction::Pressed {
            match menu_interaction {
                GameOverMenuInteractions::RetryButton => {
                    load_mission_writer.write(LoadMissionMessage {
                        id: game_over_data.mission_id,
                    });
                }
                GameOverMenuInteractions::MissionsButton => {
                    next_state.set(AppState::CampaignOverview);
                }
                GameOverMenuInteractions::QuitButton => {
                    app_exit_message_writer.write(AppExit::Success);
                }
            }
        }
    }
}
