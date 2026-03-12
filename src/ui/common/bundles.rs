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

pub fn ui_button_interaction(
    mut query: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<Button>)>,
) {
    {
        for (interaction, mut color) in &mut query {
            *color = match interaction {
                Interaction::None => PRIMARY_COLOR,
                Interaction::Hovered => PRIMARY_COLOR_HOVERED,
                Interaction::Pressed => PRIMARY_COLOR_PRESSED,
            }
            .into();
        }
    }
}

pub fn ui_card_list() -> impl Bundle {
    (Node {
        width: Val::Percent(80.),
        height: Val::Percent(80.),
        flex_direction: FlexDirection::Row,
        column_gap: Val::Px(20.),
        row_gap: Val::Px(20.),
        flex_wrap: FlexWrap::Wrap,
        ..Default::default()
    },)
}
pub fn ui_card() -> impl Bundle {
    (
        BackgroundColor(CARD_BACKGROUND_COLOR),
        BorderColor::all(CARD_BORDER_COLOR),
        Node {
            height: Val::Percent(20.),
            width: Val::Percent(20.),
            justify_content: JustifyContent::FlexStart,
            border: UiRect::all(Val::Px(5.)),
            ..default()
        },
    )
}
