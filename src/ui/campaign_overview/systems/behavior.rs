use bevy::prelude::*;

use crate::{
    campaign::{
        messages::CreateCampaignMessage,
        resources::{Campaign, SquadMember},
    },
    core::states::AppState,
    mission::messages::LoadMissionMessage,
    ui::campaign_overview::{components::*, resources::SelectedMission},
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

pub fn campaign_mission_interactions(
    mut button_query: Query<
        (&Interaction, &CampaignMissionInteractions),
        (Changed<Interaction>, With<CampaignMissionInteractions>),
    >,
    mut selected_mission: ResMut<SelectedMission>,
    mut load_mission_writer: MessageWriter<LoadMissionMessage>,
) {
    for (interaction, &menu_interaction) in button_query.iter_mut() {
        if *interaction == Interaction::Pressed {
            match menu_interaction {
                CampaignMissionInteractions::SelectMission(index) => {
                    selected_mission.id = Some(index);
                }
                CampaignMissionInteractions::StartMission(id) => {
                    load_mission_writer.write(LoadMissionMessage { id });
                }
            }
        }
    }
}
