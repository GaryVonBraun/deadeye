use bevy::prelude::*;

use crate::{
    campaign::{
        messages::CreateCampaignMessage,
        resources::{Campaign, SquadMember},
    },
    core::states::AppState,
    ui::campaign_overview::components::*,
};
pub fn campaign_overview_interactions(
    mut button_query: Query<
        (&Interaction, &CampaignOverviewInteractions),
        (Changed<Interaction>, With<CampaignOverviewInteractions>),
    >,
    mut next_state: ResMut<NextState<AppState>>,
) {
    for (interaction, &menu_interaction) in button_query.iter_mut() {
        if *interaction == Interaction::Pressed {
            match menu_interaction {
                CampaignOverviewInteractions::MainMenuButton => {
                    next_state.set(AppState::MainMenu);
                }
                CampaignOverviewInteractions::ShopMenuButton => {
                    warn!("No shop implemented yet!")
                }
            }
        }
    }
}

pub fn campaign_squad_interactions(
    mut button_query: Query<
        (&Interaction, &CampaignSquadInteractions),
        (Changed<Interaction>, With<CampaignSquadInteractions>),
    >,
    mut campaign: ResMut<Campaign>,
) {
    for (interaction, &menu_interaction) in button_query.iter_mut() {
        if *interaction == Interaction::Pressed {
            match menu_interaction {
                CampaignSquadInteractions::AddMemberButton => {
                    campaign.squad.push(SquadMember {
                        name: "Ben".to_string(),
                    });
                }
                CampaignSquadInteractions::RemoveMemberButton(index) => {
                    campaign.squad.remove(index);
                }
            }
        }
    }
}
