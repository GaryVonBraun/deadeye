use crate::ui::{
    common::{
        button::UiButton,
        styles::{CARD_BACKGROUND_COLOR, CARD_BORDER_COLOR},
    },
    hud::components::{Hud, HudHealthBar, VictoryMenuInteractions},
};
use bevy::prelude::*;
pub fn spawn_hud(mut commands: Commands, asset_server: Res<AssetServer>) {
    build_hud(&mut commands, &asset_server);
}

pub fn build_hud(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    println!("Building Hud");
    let hud_entity = commands
        .spawn((
            Node {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                ..Default::default()
            },
            Hud,
        ))
        .with_children(|parent| {
            parent.spawn((
                BackgroundColor(CARD_BACKGROUND_COLOR),
                BorderColor::all(CARD_BORDER_COLOR),
                Node {
                    width: Val::Percent(10.),
                    height: Val::Percent(2.),
                    border: UiRect::all(Val::Px(1.)),
                    ..Default::default()
                },
                HudHealthBar { value: 0. },
            ));
        })
        .id();
    hud_entity
}

pub fn despawn_hud(mut commands: Commands, victory_menu_query: Query<Entity, With<Hud>>) {
    let Ok(entity) = victory_menu_query.single() else {
        return;
    };

    commands.entity(entity).despawn();
}
