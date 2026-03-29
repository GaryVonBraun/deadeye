use bevy::prelude::*;

use crate::ui::common::styles::*;

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
            height: Val::Percent(25.),
            width: Val::Percent(20.),
            justify_content: JustifyContent::FlexStart,
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(10.),
            border: UiRect::all(Val::Px(5.)),
            ..default()
        },
    )
}
