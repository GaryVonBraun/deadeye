use bevy::prelude::*;

use crate::{
    campaign::messages::CreateCampaignMessage,
    core::states::AppState,
    ui::main_menu::components::{MainMenuInteractions, MainMenuStatChild, MainMenuStats},
};
use chrono::Local as Chrono;
use pretty_num::PrettyNumber;
use rand::RngExt;
use rand::{SeedableRng, rngs::StdRng};
pub fn main_menu_interactions(
    mut button_query: Query<
        (&Interaction, &MainMenuInteractions),
        (Changed<Interaction>, With<MainMenuInteractions>),
    >,
    mut next_state: ResMut<NextState<AppState>>,
    mut app_exit_message_writer: MessageWriter<AppExit>,
    mut create_campaign_writer: MessageWriter<CreateCampaignMessage>,
) {
    for (interaction, &menu_interaction) in button_query.iter_mut() {
        if *interaction == Interaction::Pressed {
            match menu_interaction {
                MainMenuInteractions::ContinueButton => {
                    // next_state.set(AppState::MissionMenu);
                    info!("yea so this shit not implemented yet")
                }
                MainMenuInteractions::LoadButton => {
                    next_state.set(AppState::CampaignMenu);
                }
                MainMenuInteractions::MissionsButton => {
                    next_state.set(AppState::MissionMenu);
                }
                MainMenuInteractions::SettingsButton => {
                    //TEMPORARY - currently settings don't exist so its placeholder
                    // next_state.set(AppState::MissionMenu);
                }
                MainMenuInteractions::QuitButton => {
                    app_exit_message_writer.write(AppExit::Success);
                }
                MainMenuInteractions::NewCampaignButton => {
                    create_campaign_writer.write(CreateCampaignMessage);
                }
            }
        }
    }
}

const OUTBREAK_START: i64 = 1735689600;
const WORLD_POPULATION: f64 = 8_000_000_000.0;
const INFECTED_BASE: f64 = 7_200_000_000.0;
const NEUTRALIZED_BASE: f64 = 100.0;
const NEUTRALIZED_RATE: f64 = 0.01;
pub fn ui_menu_stat_update(
    mut query: Query<(&Children, &MainMenuStats)>,
    mut text_query: Query<(&mut Text, &MainMenuStatChild)>,
) {
    let now = Chrono::now();
    {
        for (children, stat) in &mut query {
            let now_secs = now.timestamp();
            let elapsed = (now_secs - OUTBREAK_START) as f64;

            // seed the rng from the current second — same globally

            let mut rng = StdRng::seed_from_u64(now_secs as u64);

            // base rate decays over time as remaining population shrinks
            let remaining_ratio = (WORLD_POPULATION - INFECTED_BASE - elapsed * 10.0).max(0.0)
                / (WORLD_POPULATION - INFECTED_BASE);
            let max_change = (10.0 * remaining_ratio) as u64;

            // deterministic random between 0 and max_change

            let change = if max_change > 0 {
                rng.random_range(0..=max_change)
            } else {
                0
            };

            // total infected is the sum of all past changes
            // we derive this from elapsed rather than accumulating
            let infected = INFECTED_BASE + elapsed * 10.0 * remaining_ratio;
            let values = match stat {
                // i added timestamp as a test
                // but this should reflect a value for Change and Value so likely a tuple
                MainMenuStats::Infected => (
                    (infected as i64).pretty_format(),
                    format!("+{}/SEC", (change as i64).pretty_format()),
                ),
                MainMenuStats::Survivors => {
                    let survivors = (WORLD_POPULATION - infected - 1000.0).max(0.0);

                    let change_survivors = if max_change > 0 {
                        rng.random_range(0..=max_change)
                    } else {
                        0
                    };

                    (
                        (survivors as i64).pretty_format(),
                        format!("-{}/SEC", (change_survivors as i64).pretty_format()),
                    )
                }
                MainMenuStats::Neutralized => {
                    let neutralized = NEUTRALIZED_BASE + elapsed * NEUTRALIZED_RATE;
                    let neutralized_change = rng.random_range(0..=1i64);

                    (
                        (neutralized as i64).pretty_format(),
                        format!("+{}/SEC", neutralized_change.pretty_format()),
                    )
                }
                MainMenuStats::Deployed => {
                    let minute_seed = (now_secs / 60) as u64;
                    let mut squads_rng = StdRng::seed_from_u64(minute_seed);
                    let squads = squads_rng.random_range(12..=18u64);
                    ((squads as i64).pretty_format(), "Global".to_string())
                }
            };

            for child in children.iter() {
                if let Ok((mut text, child_type)) = text_query.get_mut(child) {
                    match child_type {
                        // values get applied depending on what type if ui element it is
                        MainMenuStatChild::Value => text.0 = values.0.clone(),
                        MainMenuStatChild::Change => {
                            text.0 = values.1.clone();
                        }
                    }
                }
            }
        }
    }
}
