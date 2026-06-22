use bevy::{prelude::*, text::LineBreak::NoWrap};
use bevy_egui::egui::{TextWrapMode, TextureWrapMode};

use crate::ui::common::{components::UiVariant, styles::SECONDARY_COLOR};

#[derive(Component)]
pub struct MenuButtonSubLabel;

#[derive(Component)]
pub struct MenuButtonLabel;

#[derive(Debug, Component)]
pub struct UiMenuButton {
    label: String,
    sub_label: String,
    variant: UiVariant,
}

impl UiMenuButton {
    pub fn new(label: String, sub_label: String) -> Self {
        UiMenuButton {
            label,
            sub_label,
            variant: UiVariant::default(),
        }
    }

    pub fn variant(mut self, variant: UiVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn spawn(self, parent: &mut ChildSpawnerCommands, interaction: impl Component) -> Entity {
        parent
            .spawn((
                Node {
                    width: Val::Percent(100.),
                    justify_content: JustifyContent::Center,
                    flex_direction: FlexDirection::Column,
                    padding: UiRect {
                        left: Val::Px(50.),
                        right: Val::Px(50.),
                        top: Val::Px(20.),
                        bottom: Val::Px(20.),
                    },
                    border: UiRect::all(Val::Px(2.)).with_left(Val::Px(7.)),
                    ..default()
                },
                Button,
                UiMenuButton {
                    label: self.label.clone(),
                    sub_label: self.sub_label.clone(),
                    variant: self.variant,
                },
                BorderColor::all(SECONDARY_COLOR),
                BackgroundColor::DEFAULT,
                self.variant,
                interaction,
            ))
            .with_children(|p| {
                p.spawn((
                    Text::new(self.label),
                    TextFont::from_font_size(60.),
                    MenuButtonLabel,
                ));
                p.spawn((
                    Text::new(self.sub_label),
                    TextLayout::no_wrap(),
                    TextFont::from_font_size(20.),
                    TextColor::from(SECONDARY_COLOR),
                    MenuButtonSubLabel,
                ));
            })
            .id()
    }
}

pub fn ui_menu_button_interaction(
    mut query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
            &UiVariant,
        ),
        (Changed<Interaction>, With<Button>, With<UiMenuButton>),
    >,
    mut text_query: Query<&mut TextColor, With<MenuButtonLabel>>,
) {
    {
        for (interaction, mut background_color, mut border_color, children, variant) in &mut query {
            *background_color = match interaction {
                Interaction::None => SECONDARY_COLOR.with_alpha(0.),
                Interaction::Hovered => SECONDARY_COLOR.with_alpha(0.12),
                Interaction::Pressed => SECONDARY_COLOR.with_alpha(0.22),
            }
            .into();

            *border_color = match interaction {
                Interaction::None => BorderColor::all(Color::default().with_alpha(0.)),
                Interaction::Hovered => {
                    let mut border = BorderColor::all(SECONDARY_COLOR);
                    border.left = UiVariant::base_color(*variant);
                    border
                }
                Interaction::Pressed => {
                    let mut border = BorderColor::all(SECONDARY_COLOR.darker(0.05));
                    border.left = UiVariant::base_color(*variant).darker(0.2);
                    border
                }
            };

            for child in children.iter() {
                if let Ok(mut color) = text_query.get_mut(child) {
                    *color = match interaction {
                        Interaction::None => SECONDARY_COLOR,
                        Interaction::Hovered => SECONDARY_COLOR.lighter(0.80),
                        Interaction::Pressed => SECONDARY_COLOR.darker(0.01),
                    }
                    .into()
                }
            }
        }
    }
}
