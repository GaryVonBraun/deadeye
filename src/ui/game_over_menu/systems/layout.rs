use crate::ui::{
    common::button::UiButton,
    game_over_menu::components::{GameOverMenu, GameOverMenuInteractions},
};
use bevy::prelude::*;
pub fn spawn_game_over_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    build_game_over_menu(&mut commands, &asset_server);
}

pub fn build_game_over_menu(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    println!("Building game over menu");
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
            GameOverMenu,
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
                    parent.spawn((Text::new("Mission Failed".to_string()), TextColor::WHITE));
                });
            UiButton::new("retry".to_string()).spawn(parent, GameOverMenuInteractions::RetryButton);
            UiButton::new("missions".to_string())
                .spawn(parent, GameOverMenuInteractions::MissionsButton);
            UiButton::new("rage quit".to_string())
                .spawn(parent, GameOverMenuInteractions::QuitButton);
        })
        .id();
    game_over_menu_entity
}

pub fn despawn_game_over_menu(
    mut commands: Commands,
    game_over_menu_query: Query<(Entity, &GameOverMenu)>,
) {
    let Ok(game_over_menu_entity) = game_over_menu_query.single() else {
        return;
    };

    commands.entity(game_over_menu_entity.0).despawn();
}
