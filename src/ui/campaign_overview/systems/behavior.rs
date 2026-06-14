use bevy::prelude::*;

use crate::{
    campaign::messages::CreateCampaignMessage, core::states::AppState,
    ui::campaign_overview::components::CampaignOverviewInteractions,
};
pub fn campaign_overview_interactions(
    mut button_query: Query<
        (&Interaction, &CampaignOverviewInteractions),
        (Changed<Interaction>, With<CampaignOverviewInteractions>),
    >,
    mut next_state: ResMut<NextState<AppState>>,
    mut app_exit_message_writer: MessageWriter<AppExit>,
    mut create_campaign_writer: MessageWriter<CreateCampaignMessage>,
) {
    for (interaction, &menu_interaction) in button_query.iter_mut() {
        if *interaction == Interaction::Pressed {
            match menu_interaction {
                CampaignOverviewInteractions::MainMenuButton => {
                    next_state.set(AppState::MainMenu);
                }
            }
        }
    }
}
