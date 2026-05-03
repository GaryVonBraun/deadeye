use bevy::{
    image::{ImageFilterMode, ImageLoaderSettings, ImageSampler, ImageSamplerDescriptor},
    prelude::*,
};

use crate::{
    map::{
        io::{operations::*, paths::*},
        messages::{DeleteMapMessage, LoadMapMessage},
        resources::ActiveMap,
    },
    props::resources::ActiveMapProps,
};

pub fn delete_map_message(mut message_reader: MessageReader<DeleteMapMessage>) {
    for message in message_reader.read() {
        remove_map(message.id);
    }
}

pub fn load_map_data(
    mut message_reader: MessageReader<LoadMapMessage>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    //NOTE - currently just taking the first message to prevent loading and unloading if multiple messages are present
    for message in message_reader.read() {
        // despawn all existing maps
        //NOTE - technically it should not be possible to have multiple maps, but its for safety

        let Ok(map) = read_map_data(&message.id) else {
            error!("failed to load world map with id: {:?}", message.id);
            continue;
        };

        let Ok(tileset) = read_tileset(tileset_path(map.tileset_name.clone())) else {
            info!("failed to load tileset needed for loading map");
            continue;
        };

        let handle = asset_server.load_with_settings(
            &tileset.atlas,
            |settings: &mut ImageLoaderSettings| {
                settings.sampler = ImageSampler::Descriptor(ImageSamplerDescriptor {
                    min_filter: ImageFilterMode::Nearest,
                    mag_filter: ImageFilterMode::Nearest,
                    mipmap_filter: ImageFilterMode::Nearest,
                    ..default()
                });
            },
        );

        commands.insert_resource(ActiveMap {
            map,
            tileset,
            texture: handle,
        });
    }
}

pub fn save_map(active_map_props: Res<ActiveMapProps>, mut active_map: ResMut<ActiveMap>) {
    info!("saving map with id: {}", active_map.map.id);

    //NOTE - this is a little stinky because we have essentially duplicate data, will probably not fix soon
    active_map.map.placed_props = active_map_props.props.clone();

    update_map_data(&active_map.map);
}
