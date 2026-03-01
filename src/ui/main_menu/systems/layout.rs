use crate::ui::{common::bundles::ui_button_bundle, main_menu::components::{MainMenu, MainMenuInteractions}};
use bevy::prelude::*;
pub fn spawn_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    build_main_menu(&mut commands, &asset_server);
}

pub fn build_main_menu(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    println!("Building main menu");
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
            MainMenu,
        ))
        .with_children(|parent| {
            // title
            parent
                .spawn((Node {
                    width: Val::Px(300.),
                    height: Val::Px(120.),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..Default::default()
                },))
                .with_children(|parent| {
                    // title
                    parent.spawn((Text::new("STFU".to_string()), TextColor::WHITE));
                });
            // play button
            parent
                .spawn((
                    ui_button_bundle(),
                    MainMenuInteractions::PlayButton,
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("play".to_string()),
                        TextColor::from(TextColor::WHITE),
                    ));
                });
            // quit button
            parent
                .spawn((
                    ui_button_bundle(),
                    MainMenuInteractions::SettingsButton,
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("settings".to_string()),
                        TextColor::from(TextColor::WHITE),
                    ));
                });
            parent
                .spawn((
                    ui_button_bundle(),
                    MainMenuInteractions::QuitButton,
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("quit".to_string()),
                        TextColor::from(TextColor::WHITE),
                    ));
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
