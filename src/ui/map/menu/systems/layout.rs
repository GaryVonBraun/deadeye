use crate::ui::{
    common::{
        bundles::ui_card_list,
        button::{UiButton, UiButtonVariant},
    },
    map::menu::components::{MapListUi, MapMenuInteractions, MapMenuUi},
};
use bevy::prelude::*;
pub fn spawn_map_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    build_map_menu(&mut commands, &asset_server);
}

pub fn build_map_menu(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    commands
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
            MapMenuUi,
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
                    parent.spawn((Text::new("Maps".to_string()), TextColor::WHITE));
                });

            UiButton::new("New map".to_string())
                .variant(UiButtonVariant::Success)
                .spawn(parent, MapMenuInteractions::New);
            // card that will hold the maps
            parent.spawn((ui_card_list(), MapListUi));

            UiButton::new("Back".to_string())
                .variant(UiButtonVariant::Primary)
                .spawn(parent, MapMenuInteractions::Back);
        })
        .id()
}

pub fn despawn_map_menu(mut commands: Commands, map_menu_query: Query<(Entity, &MapMenuUi)>) {
    let Ok(map_menu_entity) = map_menu_query.single() else {
        return;
    };

    commands.entity(map_menu_entity.0).despawn();
}
