use bevy::{prelude::*, ui::Node};

use crate::{
    mission::io::operations::read_missions_manifest,
    ui::{
        common::{bundles::ui_card, button::UiButton, components::UiVariant},
        missions_menu::components::{MissionListInteractions, MissionListUi},
    },
};

pub fn populate_mission_list(mut commands: Commands, query: Query<Entity, With<MissionListUi>>) {
    for entity in query.iter() {
        let Ok(manifest) = read_missions_manifest() else {
            error!("no mission manifest found");
            return;
        };

        commands.entity(entity).despawn_children();

        let mut entries: Vec<Entity> = vec![];

        for mission in manifest.missions {
            entries.push(
                commands
                    .spawn(ui_card())
                    .with_children(|parent| {
                        // title
                        parent
                            .spawn((Node {
                                width: Val::Px(300.),
                                height: Val::Px(120.),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,

                                ..Default::default()
                            },))
                            .with_children(|parent| {
                                // title
                                parent.spawn((
                                    Text::new(format!("mission name: {}", mission.name)),
                                    TextColor::from(TextColor::WHITE),
                                ));
                            });
                        UiButton::new("Play".to_string())
                            .variant(UiVariant::Primary)
                            .spawn(parent, MissionListInteractions::Play(mission.id));

                        UiButton::new("Edit".to_string())
                            .variant(UiVariant::Primary)
                            .spawn(parent, MissionListInteractions::Edit(mission.id));

                        UiButton::new("Delete".to_string())
                            .variant(UiVariant::Danger)
                            .spawn(
                                parent,
                                MissionListInteractions::Delete {
                                    mission_id: mission.id,
                                    map_id: mission.map_id,
                                },
                            );

                        // card that will hold the missions
                    })
                    .id(),
            );
        }
        commands.entity(entity).add_children(&entries);
    }
}
