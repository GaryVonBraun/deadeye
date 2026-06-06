use bevy::prelude::*;

use crate::{core::states::AppState, ui::campaign_menu::components::CampaignMenuInteractions};

pub fn campaign_menu_interactions(
    mut button_query: Query<
        (&Interaction, &CampaignMenuInteractions),
        (Changed<Interaction>, With<CampaignMenuInteractions>),
    >,
    mut next_state: ResMut<NextState<AppState>>,
) {
    for (interaction, &menu_interaction) in button_query.iter_mut() {
        if *interaction == Interaction::Pressed {
            match menu_interaction {
                CampaignMenuInteractions::BackButton => {
                    next_state.set(AppState::MainMenu);
                }
            }
        }
    }
}
