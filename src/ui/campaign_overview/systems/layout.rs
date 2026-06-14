use crate::ui::{
    campaign_overview::components::{CampaignOverview, CampaignOverviewInteractions},
    common::{button::UiButton, styles::BACKGROUND_PANEL_COLOR},
};
use bevy::prelude::*;
pub fn spawn_campaign_overview(mut commands: Commands, asset_server: Res<AssetServer>) {
    build_campaign_overview(&mut commands, &asset_server);
}

pub fn build_campaign_overview(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    println!("Building campaign overview");

    let main_menu_entity = commands
        .spawn((
            Node {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::End,
                ..Default::default()
            },
            BackgroundColor::from(BACKGROUND_PANEL_COLOR),
            CampaignOverview,
        ))
        .with_children(|p| {
            UiButton::new("main menu".to_string())
                .spawn(p, CampaignOverviewInteractions::MainMenuButton);
        })
        .id();
    main_menu_entity
}

pub fn despawn_campaign_overview(
    mut commands: Commands,
    main_menu_query: Query<(Entity, &CampaignOverview)>,
) {
    let Ok(main_menu_entity) = main_menu_query.single() else {
        return;
    };

    commands.entity(main_menu_entity.0).despawn();
}
