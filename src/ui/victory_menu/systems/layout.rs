use crate::ui::{
    common::button::UiButton,
    victory_menu::components::{VictoryMenu, VictoryMenuInteractions},
};
use bevy::prelude::*;
pub fn spawn_victory_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    build_victory_menu(&mut commands, &asset_server);
}

pub fn build_victory_menu(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    println!("Building victory menu");
    let game_over_menu_entity = commands
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
            VictoryMenu,
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
                    parent.spawn((Text::new("Mission Success".to_string()), TextColor::WHITE));
                });
            UiButton::new("missions".to_string())
                .spawn(parent, VictoryMenuInteractions::MissionsButton);
            UiButton::new("rage quit".to_string())
                .spawn(parent, VictoryMenuInteractions::QuitButton);
        })
        .id();
    game_over_menu_entity
}

pub fn despawn_victory_menu(
    mut commands: Commands,
    victory_menu_query: Query<Entity, With<VictoryMenu>>,
) {
    let Ok(entity) = victory_menu_query.single() else {
        return;
    };

    commands.entity(entity).despawn();
}
