use bevy::prelude::*;

use crate::ui::common::styles::*;

#[derive(Component, Debug)]
pub struct MainMenu;

#[derive(Component, Debug, Clone, Copy)]
pub enum MainMenuInteractions {
    NewCampaignButton,
    ContinueButton,
    LoadButton,
    SettingsButton,
    MissionsButton,
    QuitButton,
}

#[derive(Component, Debug)]
pub enum MainMenuStats {
    Infected,
    Survivors,
    Neutralized,
    Deployed,
}

#[derive(Component, Debug)]
pub enum MainMenuStatChild {
    Value,
    Change,
}

impl MainMenuStats {
    pub fn spawn(self, p: &mut ChildSpawnerCommands) {
        let stat_value = match self {
            MainMenuStats::Infected => ("Infected", DANGER_COLOR),
            MainMenuStats::Survivors => ("Survivors", WARN_COLOR),
            MainMenuStats::Neutralized => ("Neutralized", PRIMARY_COLOR),
            MainMenuStats::Deployed => ("Deployed", TEXT_SECONDARY_COLOR),
        };

        p.spawn((Node {
            flex_direction: FlexDirection::Column,
            ..Default::default()
        },))
            .with_children(|p| {
                p.spawn((
                    Text::new(stat_value.0.to_string()),
                    TextColor::from(TEXT_SECONDARY_COLOR),
                    TextFont::from_font_size(30.),
                ));

                p.spawn((
                    self,
                    Node {
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        column_gap: Val::Px(5.),
                        ..Default::default()
                    },
                ))
                .with_children(|p| {
                    p.spawn((
                        Text::new("--".to_string()),
                        TextColor::from(stat_value.1),
                        TextFont::from_font_size(50.),
                        MainMenuStatChild::Value,
                    ));

                    p.spawn((
                        Text::new(" --".to_string()),
                        TextColor::from(stat_value.1),
                        TextFont::from_font_size(15.),
                        MainMenuStatChild::Change,
                    ));
                });
            });
    }
}
