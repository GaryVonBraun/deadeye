use bevy::{prelude::*, ui::Node};

use crate::{
    campaign::resources::Campaign,
    ui::{
        campaign_overview::components::{CampaignSquadInteractions, SquadMemberList},
        common::{button::UiButton, components::UiVariant, styles::BORDER_STRONG_COLOR},
    },
};

pub fn populate_squad_member_list(
    mut commands: Commands,
    query: Query<Entity, With<SquadMemberList>>,
    campaign: Res<Campaign>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_children();

        let mut entries: Vec<Entity> = vec![];
        for (index, squad_member) in campaign.squad.iter().enumerate() {
            entries.push(
                commands
                    .spawn(Node::default())
                    .with_children(|parent| {
                        // title
                        parent
                            .spawn((
                                Node {
                                    width: Val::Px(300.),
                                    height: Val::Px(120.),
                                    justify_content: JustifyContent::Center,
                                    border: UiRect::all(Val::Px(2.)),
                                    align_items: AlignItems::Center,

                                    ..Default::default()
                                },
                                BorderColor::from(BORDER_STRONG_COLOR),
                            ))
                            .with_children(|p| {
                                // title
                                p.spawn((
                                    Text::new(format!("name: {}", squad_member.name)),
                                    TextColor::from(TextColor::WHITE),
                                ));

                                UiButton::new("dismiss".to_string())
                                    .variant(UiVariant::Danger)
                                    .spawn(p, CampaignSquadInteractions::RemoveMemberButton(index));
                            });

                        // card that will hold the missions
                    })
                    .id(),
            );
        }
        entries.push(
            commands
                .spawn(Node::default())
                .with_children(|p| {
                    UiButton::new("add".to_string())
                        .variant(UiVariant::Primary)
                        .spawn(p, CampaignSquadInteractions::AddMemberButton);
                })
                .id(),
        );

        commands.entity(entity).add_children(&entries);
    }
}
