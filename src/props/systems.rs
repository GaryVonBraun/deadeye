use bevy::prelude::*;

use crate::{
    map::io::operations::read_map_data,
    props::{
        components::Prop,
        io::operations::read_prop_definitions,
        messages::{LoadPropsMessage, RemovePropMessage, SpawnPropMessage},
    },
};

pub fn load_map_props(
    props_query: Query<Entity, With<Prop>>,
    mut commands: Commands,
    mut load_props_reader: MessageReader<LoadPropsMessage>,
    asset_server: Res<AssetServer>,
) {
    let Ok(prop_definitions) = read_prop_definitions() else {
        error!("Failed to find prop definitions needed to load props");
        return;
    };

    for message in load_props_reader.read() {
        // just in case despawn pre-existing props
        for prop_entity in props_query.iter() {
            commands.entity(prop_entity).despawn();
        }

        let Ok(map_data) = read_map_data(&message.id) else {
            error!(
                "Failed to find map data needed to load props, map: {:?}",
                message.id
            );
            return;
        };

        for placed_prop in map_data.placed_props {
            // we find the definition for the placed prop
            let Some(prop_definition) = prop_definitions
                .props
                .iter()
                .find(|definition| definition.name == placed_prop.definition_name)
            else {
                error!("Failed to find definition for placed prop");
                //TODO - perhaps in the future we can spawn the prop but show a missing texture
                return;
            };

            commands.spawn((
                Prop {
                    size: prop_definition.size,
                },
                Sprite::from_image(
                    asset_server.load(format!("props/{}.png", prop_definition.sprite)),
                ),
                Transform::from_xyz(placed_prop.position.x, placed_prop.position.y, 0.),
            ));
        }
    }
}

pub fn spawn_prop(
    mut spawn_prop_reader: MessageReader<SpawnPropMessage>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let Ok(prop_definitions) = read_prop_definitions() else {
        error!("Failed to find prop definitions needed to spawn props");
        return;
    };
    for message in spawn_prop_reader.read() {
        //FIXME - this is a repeated function, perhaps we can shorten it
        let Some(prop_definition) = prop_definitions
            .props
            .iter()
            .find(|definition| definition.name == message.name)
        else {
            error!("Failed to find definition for placed prop");
            //TODO - perhaps in the future we can spawn the prop but show a missing texture
            return;
        };

        commands.spawn((
            Prop {
                size: prop_definition.size,
            },
            Sprite::from_image(asset_server.load(format!("props/{}.png", prop_definition.sprite))),
            Transform::from_xyz(message.position.x, message.position.y, 0.),
        ));
    }
}

pub fn remove_prop(
    query: Query<(Entity, &Transform, &Prop)>,
    mut remove_prop_reader: MessageReader<RemovePropMessage>,
    mut commands: Commands,
) {
    for message in remove_prop_reader.read() {
        for (entity, transform, prop) in query.iter() {
            let pos = transform.translation.truncate();
            let half = prop.size / 2.0;

            let hit = pos.x >= message.position.x - half.x
                && pos.x <= message.position.x + half.x
                && pos.y >= message.position.y - half.y
                && pos.y <= message.position.y + half.y;

            if hit {
                commands.entity(entity).despawn();
            }
        }
    }
}
