use bevy::prelude::*;

use crate::map::{
    components::MissionMap,
    io::{operations::*, paths::*},
    messages::{DeleteMapMessage, LoadMapMessage},
    resources::ActiveMap,
};

pub fn delete_map_message(mut message_reader: MessageReader<DeleteMapMessage>) {
    for message in message_reader.read() {
        remove_map(message.id);
    }
}

pub fn load_map_data(
    mut message_reader: MessageReader<LoadMapMessage>,
    map_query: Query<(Entity, &MissionMap), With<MissionMap>>,
    mut commands: Commands,
) {
    //NOTE - currently just taking the first message to prevent loading and unloading if multiple messages are present
    for message in message_reader.read() {
        // despawn all existing maps
        //NOTE - technically it should not be possible to have multiple maps, but its for safety
        for (map_entity, map_info) in map_query.iter() {
            info!("despawning {:?}", map_info.name);
            commands.entity(map_entity).despawn();
        }

        let Ok(map) = read_map_data(&message.id) else {
            error!("failed to load world map with id: {:?}", message.id);
            continue;
        };

        let Ok(tileset) = read_tileset(tileset_path(map.tileset_name.clone())) else {
            info!("failed to load tileset needed for loading map");
            continue;
        };
        commands.insert_resource(ActiveMap { map, tileset });
    }
}

pub fn save_map(active_map: Res<ActiveMap>) {
    info!("saving map with id: {}", active_map.map.id);
    update_map_data(&active_map.map);
}