use bevy::prelude::*;

use crate::ui::common::styles::*;

#[derive(Component, Debug, Clone, Copy, Default)]
pub enum UiButtonVariant {
    Primary,
    #[default]
    Secondary,
    Success,
    Warn,
    Danger,
}

impl UiButtonVariant {
    pub fn base_color(variant: UiButtonVariant) -> Color {
        match variant {
            UiButtonVariant::Primary => PRIMARY_COLOR,
            UiButtonVariant::Secondary => SECONDARY_COLOR,
            UiButtonVariant::Success => SUCCESS_COLOR,
            UiButtonVariant::Warn => WARN_COLOR,
            UiButtonVariant::Danger => DANGER_COLOR,
        }
    }
}

#[derive(Component)]
pub struct ButtonLabel;

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
                    border: UiRect::all(Val::Px(1.)),
                    border_radius: BorderRadius::all(Val::Px(5.)),
                    ..default()
                },
                Button,
                BorderColor::all(UiButtonVariant::base_color(self.variant)),
                BackgroundColor::DEFAULT,
                self.variant,
                interaction,
            ))
            .with_children(|p| {
                p.spawn((Text::new(self.label), ButtonLabel));
            })
            .id()
    }
}

pub fn ui_button_interaction(
    mut query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
            &UiButtonVariant,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut TextColor, With<ButtonLabel>>,
) {
    {
        for (interaction, mut background_color, mut border_color, children, variant) in &mut query {
            *background_color = match interaction {
                Interaction::None => UiButtonVariant::base_color(*variant).with_alpha(0.),
                Interaction::Hovered => UiButtonVariant::base_color(*variant).with_alpha(0.12),
                Interaction::Pressed => UiButtonVariant::base_color(*variant).with_alpha(0.22),
            }
            .into();

            *border_color = match interaction {
                Interaction::None => UiButtonVariant::base_color(*variant),
                Interaction::Hovered => UiButtonVariant::base_color(*variant).lighter(0.08),
                Interaction::Pressed => UiButtonVariant::base_color(*variant).darker(0.01),
            }
            .into();
            for child in children.iter() {
                if let Ok(mut color) = text_query.get_mut(child) {
                    *color = match interaction {
                        Interaction::None => UiButtonVariant::base_color(*variant),
                        Interaction::Hovered => UiButtonVariant::base_color(*variant).lighter(0.08),
                        Interaction::Pressed => UiButtonVariant::base_color(*variant).darker(0.01),
                    }
                    .into()
                }
            }
        }
    }
}
