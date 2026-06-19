use bevy::{prelude::*, ui::Node};

use crate::{
    campaign::resources::Campaign,
    mission::io::operations::{read_mission_data, read_missions_manifest},
    ui::{
        campaign_overview::{
            components::{
                CampaignMissionInteractions, CampaignSquadInteractions, MissionEntry,
                SquadMemberList, UiMissionBriefing, UiMissionList,
            },
            resources::SelectedMission,
        },
        common::{
            button::UiButton,
            components::UiVariant,
            styles::{BORDER_STRONG_COLOR, PRIMARY_COLOR},
        },
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

pub fn populate_mission_list(mut commands: Commands, query: Query<Entity, With<UiMissionList>>) {
    for entity in query.iter() {
        let Ok(manifest) = read_missions_manifest() else {
            error!("no mission manifest found");
            return;
        };

        commands.entity(entity).despawn_children();

        let mut entries: Vec<Entity> = vec![];

        for (index, mission) in manifest.missions.iter().enumerate() {
            entries.push(
                commands
                    .spawn((
                        Node {
                            width: Val::Percent(100.),
                            height: Val::Px(120.),
                            border: UiRect::all(Val::Px(1.)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..Default::default()
                        },
                        Button,
                        CampaignMissionInteractions::SelectMission(mission.id),
                        MissionEntry { id: mission.id },
                        BorderColor::from(BORDER_STRONG_COLOR),
                    ))
                    .with_children(|parent| {
                        // title
                        parent.spawn((
                            Text::new(format!("mission name: {}", mission.name)),
                            TextColor::from(TextColor::WHITE),
                        ));
                    })
                    .id(),
            );
        }
        commands.entity(entity).add_children(&entries);
    }
}

pub fn selected_mission_system(
    mut query: Query<(&mut Node, &mut BorderColor, &MissionEntry), With<MissionEntry>>,
    selected_mission: Res<SelectedMission>,
) {
    let Some(id) = selected_mission.id else {
        return;
    };
    for (mut node, mut border, mission_entry) in query.iter_mut() {
        if mission_entry.id == id {
            node.border = UiRect::all(Val::Px(2.));
            border.set_all(PRIMARY_COLOR);
        } else {
            node.border = UiRect::all(Val::Px(1.));
            border.set_all(BORDER_STRONG_COLOR);
        }
    }
}

pub fn populate_mission_briefing(
    query: Query<Entity, With<UiMissionBriefing>>,
    mut commands: Commands,
    selected_mission: Res<SelectedMission>,
) {
    let Some(id) = selected_mission.id else {
        return;
    };

    let Ok(mission) = read_mission_data(&id) else {
        return;
    };

    for entity in query.iter() {
        commands.entity(entity).despawn_children();

        commands.entity(entity).with_children(|p| {
            p.spawn(Text::new(&mission.name));
            UiButton::new("start mission".to_string())
                .variant(UiVariant::Primary)
                .spawn(p, CampaignMissionInteractions::StartMission(id));
        });
    }
}
