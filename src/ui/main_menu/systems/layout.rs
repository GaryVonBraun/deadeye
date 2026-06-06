use crate::ui::{
    common::{
        button::{UiButton, UiButtonVariant},
        styles::BACKGROUND_PRIMARY_COLOR,
    },
    main_menu::components::{MainMenu, MainMenuInteractions},
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
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        max_width: Val::Percent(30.),

                        row_gap: Val::Vh(1.),
                        ..Default::default()
                    },
                    BackgroundColor::from(BACKGROUND_PRIMARY_COLOR),
                ))
                .with_children(|parent| {
                    // title
                    parent.spawn((
                        Text::new("Zormb Game".to_string()),
                        TextColor::WHITE,
                        TextFont::from_font_size(50.),
                    ));
                    UiButton::new("continue".to_string())
                        .spawn(parent, MainMenuInteractions::ContinueButton);
                    UiButton::new("load".to_string())
                        .spawn(parent, MainMenuInteractions::LoadButton);
                    UiButton::new("setting".to_string())
                        .variant(UiButtonVariant::Warn)
                        .spawn(parent, MainMenuInteractions::SettingsButton);
                    UiButton::new("missions".to_string())
                        .spawn(parent, MainMenuInteractions::MissionsButton);
                    UiButton::new("quit".to_string())
                        .spawn(parent, MainMenuInteractions::QuitButton);
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
