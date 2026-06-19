use bevy::prelude::*;

use crate::{
    core::states::AppState, mission::messages::LoadMissionMessage,
    ui::victory_menu::components::VictoryMenuInteractions,
};

pub fn victory_menu_interactions(
    mut button_query: Query<
        (&Interaction, &VictoryMenuInteractions),
        (Changed<Interaction>, With<VictoryMenuInteractions>),
    >,
    mut next_state: ResMut<NextState<AppState>>,
    mut app_exit_message_writer: MessageWriter<AppExit>,
    mut load_mission_writer: MessageWriter<LoadMissionMessage>,
) {
    for (interaction, &menu_interaction) in button_query.iter_mut() {
        if *interaction == Interaction::Pressed {
            match menu_interaction {
                VictoryMenuInteractions::MissionsButton => {
                    next_state.set(AppState::CampaignOverview);
                }
                VictoryMenuInteractions::QuitButton => {
                    app_exit_message_writer.write(AppExit::Success);
                }
            }
        }
    }
}
