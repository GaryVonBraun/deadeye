use bevy::prelude::*;
use bevy_egui::{EguiContexts, egui};

use crate::ui::{
    campaign_menu::CampaignMenuPlugin,
    campaign_overview::CampaignOverviewPlugin,
    common::{button::ui_button_interaction, menu_button::ui_menu_button_interaction},
    game_over_menu::GameOverMenuPlugin,
    hud::HudPlugin,
    main_menu::MainMenuPlugin,
    missions_menu::MissionDevMenuPlugin,
    victory_menu::VictoryMenuPlugin,
};

mod campaign_menu;
mod campaign_overview;
mod common;
mod game_over_menu;
mod hud;
mod main_menu;
pub mod missions_menu;
mod victory_menu;
pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            MainMenuPlugin,
            MissionDevMenuPlugin,
            GameOverMenuPlugin,
            VictoryMenuPlugin,
            HudPlugin,
            CampaignMenuPlugin,
            CampaignOverviewPlugin,
        ));
        app.insert_resource(UiScale(1.));
        app.add_systems(Update, (ui_button_interaction, ui_menu_button_interaction));
        // app.add_systems(EguiPrimaryContextPass, temporary_scale_window);
    }
}

fn temporary_scale_window(mut contexts: EguiContexts, mut ui_scale: ResMut<UiScale>) -> Result {
    egui::Window::new("Scale Ui Menu")
        .resizable(false)
        .default_pos(egui::pos2(1500.0, 16.0))
        .show(contexts.ctx_mut()?, |ui| {
            ui.horizontal(|ui| {
                ui.label("UI Scale");
                let mut scale = ui_scale.0;
                if ui
                    .add(egui::Slider::new(&mut scale, 0.5..=3.0).step_by(0.10))
                    .changed()
                {
                    ui_scale.0 = scale;
                }
            });
        });
    Ok(())
}
