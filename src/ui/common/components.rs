use bevy::prelude::*;

use crate::ui::common::styles::*;

#[derive(Component, Debug, Clone, Copy, Default)]
pub enum UiVariant {
    Primary,
    #[default]
    Secondary,
    Success,
    Warn,
    Danger,
}

impl UiVariant {
    pub fn base_color(variant: UiVariant) -> Color {
        match variant {
            UiVariant::Primary => PRIMARY_COLOR,
            UiVariant::Secondary => SECONDARY_COLOR,
            UiVariant::Success => SUCCESS_COLOR,
            UiVariant::Warn => WARN_COLOR,
            UiVariant::Danger => DANGER_COLOR,
        }
    }
}
