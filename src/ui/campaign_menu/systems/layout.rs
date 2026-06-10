use crate::ui::{
    campaign_menu::components::{CampaignMenuInteractions, MainMenu},
    common::{button::UiButton, components::UiVariant, divider::UiDivider, styles::*},
};
use bevy::prelude::*;
pub fn spawn_campaign_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    build_campaign_menu(&mut commands, &asset_server);
}

pub fn build_campaign_menu(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    println!("Building campaign menu");

    let image = asset_server.load("main_menu_placeholder.png");

    let main_menu_entity = commands
        .spawn((
            Node {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                row_gap: Val::Px(20.),
                ..Default::default()
            },
            ImageNode { image, ..default() },
            MainMenu,
        ))
        .with_children(|parent| {
            // title
            parent
                .spawn((
                    Node {
                        width: Val::Percent(30.),
                        height: Val::Percent(50.),
                        border: UiRect::all(Val::Px(1.)),
                        flex_direction: FlexDirection::Column,
                        ..Default::default()
                    },
                    BorderColor::all(BORDER_STRONG_COLOR),
                    BackgroundColor::from(BACKGROUND_PANEL_COLOR),
                ))
                .with_children(|p| {
                    // title

                    p.spawn((
                        Node {
                            width: Val::Percent(100.),
                            justify_content: JustifyContent::SpaceBetween,
                            ..Default::default()
                        },
                        BackgroundColor::from(BACKGROUND_DEEP_COLOR),
                    ))
                    .with_children(|p| {
                        UiButton::new("Back".to_string())
                            .variant(UiVariant::Secondary)
                            .spawn(p, CampaignMenuInteractions::BackButton);
                        UiDivider::horizontal();
                    });
                });
        })
        .id();
    main_menu_entity
}

pub fn despawn_campaign_menu(mut commands: Commands, main_menu_query: Query<(Entity, &MainMenu)>) {
    let Ok(main_menu_entity) = main_menu_query.single() else {
        return;
    };

    commands.entity(main_menu_entity.0).despawn();
}
