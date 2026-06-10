use crate::ui::{
    common::{components::UiVariant, divider::UiDivider, menu_button::UiMenuButton, styles::*},
    main_menu::components::{MainMenu, MainMenuInteractions, MainMenuStats},
};
use bevy::prelude::*;
pub fn spawn_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    build_main_menu(&mut commands, &asset_server);
}

pub fn build_main_menu(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    println!("Building main menu");

    let image = asset_server.load("main_menu_placeholder.png");

    let main_menu_entity = commands
        .spawn((
            Node {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::End,
                ..Default::default()
            },
            ImageNode { image, ..default() },
            MainMenu,
        ))
        .with_children(|p| {
            p.spawn((Node {
                height: Val::Percent(100.),
                ..Default::default()
            },))
                .with_children(|p| {
                    // title
                    p.spawn((
                        Node {
                            flex_direction: FlexDirection::Column,
                            align_items: AlignItems::Center,
                            // padding: UiRect::horizontal(Val::Px(20.)),
                            border: UiRect::right(Val::Px(2.)),

                            ..Default::default()
                        },
                        BorderColor::all(BORDER_STRONG_COLOR),
                        BackgroundColor::from(BACKGROUND_DEEP_COLOR),
                    ))
                    .with_children(|parent| {
                        // title
                        parent.spawn((
                            Node {
                                margin: UiRect::vertical(Val::Px(20.)),
                                ..Default::default()
                            },
                            Text::new("Dead\nSector".to_string()),
                            TextColor::from(TEXT_PRIMARY_COLOR),
                            TextFont::from_font_size(80.),
                        ));

                        UiDivider::horizontal().spawn(parent);
                        parent
                            .spawn(Node {
                                width: Val::Percent(100.),

                                flex_direction: FlexDirection::Column,
                                ..Default::default()
                            })
                            .with_children(|p| {
                                UiMenuButton::new("New".to_string(), "New Campaign".to_string())
                                    .variant(UiVariant::Primary)
                                    .spawn(p, MainMenuInteractions::NewCampaignButton);
                                UiMenuButton::new(
                                    "load".to_string(),
                                    "Select campaign".to_string(),
                                )
                                .spawn(p, MainMenuInteractions::LoadButton);
                                UiMenuButton::new(
                                    "setting".to_string(),
                                    "Audio - Controls - Visuals".to_string(),
                                )
                                .spawn(p, MainMenuInteractions::SettingsButton);
                                UiMenuButton::new(
                                    "missions".to_string(),
                                    "Edit or Play a mission".to_string(),
                                )
                                .spawn(p, MainMenuInteractions::MissionsButton);
                                UiMenuButton::new(
                                    "quit".to_string(),
                                    "Exit to desktop".to_string(),
                                )
                                .variant(UiVariant::Danger)
                                .spawn(p, MainMenuInteractions::QuitButton);
                            });
                    });
                });
            p.spawn((
                Node {
                    width: Val::Percent(100.),
                    justify_content: JustifyContent::SpaceEvenly,
                    padding: UiRect::vertical(Val::Px(10.)),
                    border: UiRect::top(Val::Px(2.)),
                    ..Default::default()
                },
                BackgroundColor::from(BACKGROUND_DEEP_COLOR),
                BorderColor::all(BORDER_STRONG_COLOR),
            ))
            .with_children(|p| {
                MainMenuStats::Infected.spawn(p);
                UiDivider::vertical().spawn(p);
                MainMenuStats::Survivors.spawn(p);
                UiDivider::vertical().spawn(p);
                MainMenuStats::Neutralized.spawn(p);
                UiDivider::vertical().spawn(p);
                MainMenuStats::Deployed.spawn(p);
            });
        })
        .id();
    main_menu_entity
}

pub fn despawn_main_menu(mut commands: Commands, main_menu_query: Query<(Entity, &MainMenu)>) {
    let Ok(main_menu_entity) = main_menu_query.single() else {
        return;
    };

    commands.entity(main_menu_entity.0).despawn();
}
