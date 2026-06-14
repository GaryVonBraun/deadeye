use bevy::prelude::*;

use crate::ui::common::components::UiVariant;

#[derive(Component)]
pub struct ButtonLabel;

#[derive(Component, Debug)]
pub struct UiButton {
    label: String,
    variant: UiVariant,
}

impl UiButton {
    pub fn new(label: String) -> Self {
        UiButton {
            label,
            variant: UiVariant::default(),
        }
    }

    pub fn variant(mut self, variant: UiVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn spawn(self, p: &mut ChildSpawnerCommands, interaction: impl Component) -> Entity {
        p.spawn((
            Node {
                height: Val::Px(80.),
                width: Val::Px(200.),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                border: UiRect::all(Val::Px(2.)),
                ..default()
            },
            Button,
            UiButton {
                label: self.label.clone(),
                variant: self.variant,
            },
            BorderColor::all(UiVariant::base_color(self.variant)),
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
            &UiVariant,
        ),
        (Changed<Interaction>, With<Button>, With<UiButton>),
    >,
    mut text_query: Query<&mut TextColor, With<ButtonLabel>>,
) {
    {
        for (interaction, mut background_color, mut border_color, children, variant) in &mut query {
            *background_color = match interaction {
                Interaction::None => UiVariant::base_color(*variant).with_alpha(0.),
                Interaction::Hovered => UiVariant::base_color(*variant).with_alpha(0.12),
                Interaction::Pressed => UiVariant::base_color(*variant).with_alpha(0.22),
            }
            .into();

            *border_color = match interaction {
                Interaction::None => UiVariant::base_color(*variant),
                Interaction::Hovered => UiVariant::base_color(*variant).lighter(0.08),
                Interaction::Pressed => UiVariant::base_color(*variant).darker(0.01),
            }
            .into();
            for child in children.iter() {
                if let Ok(mut color) = text_query.get_mut(child) {
                    *color = match interaction {
                        Interaction::None => UiVariant::base_color(*variant),
                        Interaction::Hovered => UiVariant::base_color(*variant).lighter(0.08),
                        Interaction::Pressed => UiVariant::base_color(*variant).darker(0.01),
                    }
                    .into()
                }
            }
        }
    }
}
