use crate::ui::{
    common::{components::UiVariant, menu_button::UiMenuButton, styles::*},
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
                row_gap: Val::Px(24.),
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
                        padding: UiRect::horizontal(Val::Px(20.)),
                        border: UiRect::right(Val::Px(2.)),
                        row_gap: Val::Vh(1.),
                        ..Default::default()
                    },
                    BorderColor::all(BORDER_STRONG_COLOR),
                    BackgroundColor::from(BACKGROUND_DEEP_COLOR),
                ))
                .with_children(|parent| {
                    // title
                    parent.spawn((
                        Text::new("Dead\nSector".to_string()),
                        TextColor::from(TEXT_PRIMARY_COLOR),
                        TextFont::from_font_size(70.),
                    ));
                    UiMenuButton::new("continue".to_string(), "Continue last campaign".to_string())
                        .variant(UiVariant::Primary)
                        .spawn(parent, MainMenuInteractions::ContinueButton);
                    UiMenuButton::new("load".to_string(), "Select campaign".to_string())
                        .variant(UiVariant::Primary)
                        .spawn(parent, MainMenuInteractions::LoadButton);
                    UiMenuButton::new(
                        "setting".to_string(),
                        "Audio - Controls - Visuals".to_string(),
                    )
                    .variant(UiVariant::Primary)
                    .spawn(parent, MainMenuInteractions::SettingsButton);
                    UiMenuButton::new("missions".to_string(), "Edit or Play a mission".to_string())
                        .variant(UiVariant::Primary)
                        .spawn(parent, MainMenuInteractions::MissionsButton);
                    UiMenuButton::new("quit".to_string(), "Exit to desktop".to_string())
                        .variant(UiVariant::Danger)
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
