use crate::ui::{
    common::styles::{CARD_BACKGROUND_COLOR, CARD_BORDER_COLOR},
    hud::components::{Hud, HudHealthBar, HudWaves, HudZombieCount},
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
                justify_content: JustifyContent::SpaceBetween,
                flex_direction: FlexDirection::Column,
                // align_content: AlignContent::SpaceBetween,
                ..Default::default()
            },
            Hud,
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    BackgroundColor(Color::linear_rgba(0., 0., 0., 0.3)),
                    Node {
                        width: Val::Percent(100.),
                        height: Val::Percent(3.),
                        column_gap: Val::Percent(1.),
                        ..Default::default()
                    },
                ))
                .with_children(|parent| {
                    parent
                        .spawn((Node {
                            ..Default::default()
                        },))
                        .with_children(|parent| {
                            parent.spawn((
                                Text::new("Zombie amount: 0"),
                                HudZombieCount { value: 0 },
                            ));
                        });

                    parent
                        .spawn((Node {
                            ..Default::default()
                        },))
                        .with_children(|parent| {
                            parent.spawn((Text::new("Wave: 0"), HudWaves { current_wave: 0 }));
                        });
                });
            parent
                .spawn((
                    BackgroundColor(CARD_BACKGROUND_COLOR),
                    BorderColor::all(CARD_BORDER_COLOR),
                    Node {
                        width: Val::Percent(15.),
                        height: Val::Percent(4.),
                        border: UiRect::all(Val::Px(1.)),
                        ..Default::default()
                    },
                ))
                .with_children(|parent| {
                    parent.spawn((
                        BackgroundColor(Color::linear_rgb(1., 0., 0.)),
                        Node {
                            width: Val::Percent(100.),
                            height: Val::Percent(100.),
                            ..Default::default()
                        },
                        HudHealthBar { value: 0. },
                    ));
                });
        })
        .id();
    hud_entity
}

pub fn despawn_hud(mut commands: Commands, hud_query: Query<Entity, With<Hud>>) {
    let Ok(entity) = hud_query.single() else {
        return;
    };

    commands.entity(entity).despawn();
}
