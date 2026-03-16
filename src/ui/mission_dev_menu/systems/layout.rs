use crate::ui::{
    common::{
        bundles::ui_card_list,
        button::{UiButton, UiButtonVariant},
    },
    mission_dev_menu::components::{
        MissionDevListUi, MissionDevMenuInteractions, MissionDevMenuUi,
    },
};
use bevy::prelude::*;
pub fn spawn_missions_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    build_missions_menu(&mut commands, &asset_server);
}

pub fn build_missions_menu(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
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
            MissionDevMenuUi,
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
                    parent.spawn((Text::new("Missions".to_string()), TextColor::WHITE));
                });

            UiButton::new("New Mission".to_string())
                .variant(UiButtonVariant::Success)
                .spawn(parent, MissionDevMenuInteractions::New);
            // card that will hold the missions
            parent.spawn((ui_card_list(), MissionDevListUi));

            UiButton::new("Back".to_string())
                .variant(UiButtonVariant::Primary)
                .spawn(parent, MissionDevMenuInteractions::Back);
        })
        .id()
}

pub fn despawn_missions_menu(
    mut commands: Commands,
    missions_menu_query: Query<(Entity, &MissionDevMenuUi)>,
) {
    let Ok(missions_menu_entity) = missions_menu_query.single() else {
        return;
    };

    commands.entity(missions_menu_entity.0).despawn();
}
