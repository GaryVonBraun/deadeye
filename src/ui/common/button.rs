use bevy::prelude::*;

use crate::ui::common::styles::*;

#[derive(Component, Debug, Clone, Copy, Default)]
pub enum UiButtonVariant {
    #[default]
    Primary,
    Success,
    Danger,
}

impl UiButtonVariant {
    pub fn base_color(variant: UiButtonVariant) -> Color {
        match variant {
            UiButtonVariant::Primary => PRIMARY_COLOR,
            UiButtonVariant::Success => SUCCESS_COLOR,
            UiButtonVariant::Danger => DANGER_COLOR,
        }
    }
}

pub struct UiButton {
    label: String,
    variant: UiButtonVariant,
}

impl UiButton {
    pub fn new(label: String) -> Self {
        UiButton {
            label,
            variant: UiButtonVariant::default(),
        }
    }

    pub fn variant(mut self, variant: UiButtonVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn spawn(self, parent: &mut ChildSpawnerCommands, interaction: impl Component) -> Entity {
        parent
            .spawn((
                Node {
                    height: Val::Px(80.),
                    width: Val::Px(200.),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    border: UiRect::all(Val::Vw(0.1)),
                    ..default()
                },
                Button,
                BorderColor::all(UiButtonVariant::base_color(self.variant)),
                self.variant,
                interaction,
            ))
            .with_children(|p| {
                p.spawn(Text::new(self.label));
            })
            .id()
    }
}

pub fn ui_button_interaction(
    mut query: Query<
        (&Interaction, &mut BorderColor, &UiButtonVariant),
        (Changed<Interaction>, With<Button>),
    >,
) {
    {
        for (interaction, mut color, variant) in &mut query {
            *color = match interaction {
                Interaction::None => UiButtonVariant::base_color(*variant),
                Interaction::Hovered => UiButtonVariant::base_color(*variant).lighter(0.05),
                Interaction::Pressed => UiButtonVariant::base_color(*variant).lighter(0.04),
            }
            .into();
        }
    }
}
