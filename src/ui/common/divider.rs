use bevy::prelude::*;

use crate::ui::common::styles::BORDER_STRONG_COLOR;

pub enum UiDivider {
    Vertical(f32),
    Horizontal(f32),
}

impl UiDivider {
    pub fn horizontal_with(height: f32) -> Self {
        UiDivider::Horizontal(height)
    }

    pub fn vertical_with(width: f32) -> Self {
        UiDivider::Vertical(width)
    }

    pub fn horizontal() -> Self {
        UiDivider::Horizontal(1.)
    }

    pub fn vertical() -> Self {
        UiDivider::Vertical(1.)
    }

    pub fn spawn(self, parent: &mut ChildSpawnerCommands) -> Entity {
        match self {
            UiDivider::Vertical(width) => parent
                .spawn((
                    Node {
                        height: Val::Percent(100.),
                        width: Val::Px(width),
                        ..default()
                    },
                    Button,
                    BackgroundColor::from(BORDER_STRONG_COLOR),
                ))
                .id(),
            UiDivider::Horizontal(height) => parent
                .spawn((
                    Node {
                        height: Val::Px(height),
                        width: Val::Percent(100.),
                        ..default()
                    },
                    Button,
                    BackgroundColor::from(BORDER_STRONG_COLOR),
                ))
                .id(),
        }
    }
}
