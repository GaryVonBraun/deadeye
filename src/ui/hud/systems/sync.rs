use bevy::prelude::*;

use crate::{
    combat::health::components::Health, player::components::Player,
    ui::hud::components::HudHealthBar,
};

pub fn update_health_bar(
    player_query: Query<&Health, With<Player>>,
    mut hud_query: Query<(Entity, &mut HudHealthBar)>,
    mut commands: Commands,
) {
    let Ok(player_health) = player_query.single() else {
        return;
    };

    let Ok((entity, mut hud_health)) = hud_query.single_mut() else {
        return;
    };

    if hud_health.value == player_health.current {
        return;
    }
    hud_health.value = player_health.current;

    commands.entity(entity).despawn_children();


    let health_progress = commands
        .spawn((
            BackgroundColor(Color::linear_rgb(1., 0., 0.)),
            Node {
                width: Val::Percent((player_health.current / player_health.max) * 100.),
                height: Val::Percent(100.),
                ..Default::default()
            },
        ))
        .id();

    commands.entity(entity).add_child(health_progress);
}
