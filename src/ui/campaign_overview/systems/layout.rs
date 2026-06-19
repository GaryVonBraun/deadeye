use crate::{
    campaign::resources::Campaign,
    ui::{
        campaign_overview::components::{
            CampaignOverview, CampaignOverviewInteractions, SquadMemberList,
        },
        common::{button::UiButton, components::UiVariant, styles::*},
    },
};
use bevy::prelude::*;
pub fn spawn_campaign_overview(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    campaign_info: Res<Campaign>,
) {
    build_campaign_overview(&mut commands, &asset_server, campaign_info);
}

pub fn build_campaign_overview(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    campaign_info: Res<Campaign>,
) -> Entity {
    println!("Building campaign overview");

    let main_menu_entity = commands
        .spawn((
            Node {
                flex_direction: FlexDirection::Column,
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                ..Default::default()
            },
            BackgroundColor::from(BACKGROUND_PANEL_COLOR),
            CampaignOverview,
        ))
        .with_children(
            |p: &mut bevy::ecs::relationship::RelatedSpawnerCommands<'_, ChildOf>| {
                // top bar
                p.spawn((
                    Node {
                        border: UiRect::bottom(Val::Px(2.)),
                        ..Default::default()
                    },
                    BackgroundColor::from(BACKGROUND_DEEP_COLOR),
                    BorderColor::all(BORDER_STRONG_COLOR),
                ))
                .with_children(|p| {
                    p.spawn(Text::new(format!("Moneys ${}", campaign_info.money)));
                    UiButton::new("shop".to_string())
                        .variant(UiVariant::Warn)
                        .spawn(p, CampaignOverviewInteractions::ShopMenuButton);
                });

                // menu sections
                p.spawn(Node {
                    flex_direction: FlexDirection::Row,
                    height: Val::Percent(100.),
                    ..Default::default()
                })
                .with_children(|p| {
                    menu_section("SQUAD ROSTER", p, |p| {
                        p.spawn((
                            Node {
                                flex_direction: FlexDirection::Column,
                                ..Default::default()
                            },
                            SquadMemberList,
                        ));
                    });
                    menu_section("MISSION SELECT", p, |p| {
                        UiButton::new("test2".to_string())
                            .spawn(p, CampaignOverviewInteractions::MainMenuButton);
                    });
                });

                UiButton::new("main menu".to_string())
                    .spawn(p, CampaignOverviewInteractions::MainMenuButton);
            },
        )
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

pub fn menu_section(
    label: &str,
    p: &mut ChildSpawnerCommands,
    spawn_contents: impl FnOnce(&mut ChildSpawnerCommands),
) {
    p.spawn((
        Node {
            border: UiRect::all(Val::Px(2.)),
            flex_grow: 1.,
            flex_direction: FlexDirection::Column,
            ..Default::default()
        },
        BackgroundColor::from(BACKGROUND_PANEL_COLOR),
        BorderColor::all(BORDER_STRONG_COLOR),
    ))
    .with_children(|p| {
        p.spawn((
            Text::new(label),
            Node {
                justify_self: JustifySelf::Center,
                margin: UiRect::all(Val::Px(15.)),
                ..Default::default()
            },
            TextColor::from(TEXT_PRIMARY_COLOR),
        ));
        p.spawn((
            Node {
                border: UiRect::top(Val::Px(2.)),
                flex_grow: 1.,
                ..Default::default()
            },
            BorderColor::all(BORDER_STRONG_COLOR),
        ))
        .with_children(spawn_contents);
    });
}
