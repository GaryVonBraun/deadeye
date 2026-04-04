use bevy::prelude::*;

use crate::{
    collision::components::Collision,
    core::components::GameEntity,
    map::io::operations::read_map_data,
    props::{
        bundles::PropBundle,
        components::Prop,
        io::{operations::read_prop_definitions, types::PlacedProp},
        messages::{LoadPropsMessage, RemovePropMessage, SpawnPropMessage},
        resources::ActiveMapProps,
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

        let props: Vec<PlacedProp> = map_data
            .placed_props
            .iter()
            .map(|prop| {
                let Some(prop_definition) = prop_definitions
                    .props
                    .iter()
                    .find(|definition| definition.name == prop.id)
                else {
                    warn!("Failed to find definition for placed prop called {}, missing texture placed at - x: {} y: {} ", prop.id, prop.position.x, prop.position.y);

                    let entity = commands
                        .spawn(PropBundle{
                             prop: Prop::default(), 
                             sprite: Sprite::from_image(asset_server.load("props/missing_prop_64.png")), 
                             transform: Transform::from_xyz(prop.position.x, prop.position.y, 0.), 
                             collision: Collision::default(), 
                             game_entity: GameEntity } )
                        .id();

                    return PlacedProp {
                        entity: Some(entity),
                        ..prop.clone()
                    };
                };

                let entity = commands
                        .spawn(PropBundle{
                             prop: Prop::with_size(prop_definition.size), 
                             sprite: Sprite::from_image(asset_server.load(format!("props/{}.png", prop_definition.sprite))), 
                             transform: Transform::from_xyz(prop.position.x, prop.position.y, 0.), 
                             collision: prop_definition.collision.clone(),  
                             game_entity: GameEntity } )
                             
                        .id();

                PlacedProp {
                    entity: Some(entity),
                    ..prop.clone()
                }
            })
            .collect();

        commands.insert_resource(ActiveMapProps { props });
    }
}

pub fn spawn_prop(
    mut spawn_prop_reader: MessageReader<SpawnPropMessage>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut active_map_props: ResMut<ActiveMapProps>,
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

        let entity = commands
            .spawn(PropBundle{
                 prop: Prop::with_size(prop_definition.size), 
                 sprite: Sprite::from_image(asset_server.load(format!("props/{}.png", prop_definition.sprite))), 
                 transform: Transform::from_xyz(message.position.x, message.position.y, 0.), 
                 collision: prop_definition.collision.clone(), 
                 game_entity: GameEntity } )
            .id();


        active_map_props.props.push(PlacedProp {
            id: message.name.clone(),
            position: message.position,
            entity: Some(entity),
        });
    }
}

pub fn remove_prop(
    query: Query<(Entity, &Transform, &Prop)>,
    mut remove_prop_reader: MessageReader<RemovePropMessage>,
    mut commands: Commands,
    mut active_map_props: ResMut<ActiveMapProps>,
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
                active_map_props
                    .props
                    .retain(|prop| prop.entity != Some(entity));
            }
        }
    }
}

pub fn unload_props(prop_query: Query<Entity, With<Prop>>, mut commands: Commands) {
    for entity in prop_query.iter() {
        commands.entity(entity).despawn();
    }
}
