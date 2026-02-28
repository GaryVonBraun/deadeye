use bevy::prelude::*;

use crate::ui::common::styles::*;

pub fn ui_button_bundle() -> impl Bundle {
    (
        Button,
        BackgroundColor(PRIMARY_COLOR),
        Node {
            height: Val::Px(80.),
            width: Val::Px(200.),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
    )
}

pub fn ui_button_interaction(mut query: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<Button>)>,){
    {
    for (interaction, mut color) in &mut query {
        *color = match interaction {
            Interaction::None => PRIMARY_COLOR,
            Interaction::Hovered => PRIMARY_COLOR_HOVERED,
            Interaction::Pressed => PRIMARY_COLOR_PRESSED,
        }.into();
    }}
}