use bevy::prelude::*;

use crate::{
    ui::{common::bundles::ui_card, map::menu::components::MapListUi},
    world::map::io::read_manifest,
};

pub fn populate_map_list(mut commands: Commands, query: Query<Entity, With<MapListUi>>) {
    for entity in query.iter() {
        let Ok(manifest) = read_manifest() else {
            error!("no manifest found");
            return;
        };

        let mut entries: Vec<Entity> = vec![];

        for map in manifest.maps {
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
                                    Text::new(format!("map name: {}", map.name)),
                                    TextColor::from(TextColor::WHITE),
                                ));
                            });
                        // card that will hold the maps
                    })
                    .id(),
            );
        }
        commands.entity(entity).add_children(&entries);
    }
}
