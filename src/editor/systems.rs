use bevy::{
    input::mouse::{MouseScrollUnit, MouseWheel},
    prelude::*,
};

use crate::{
    core::states::AppState,
    editor::{component::PlacementPreview, messages::*, resources::*},
    map::{
        components::MissionMapChunk,
        messages::{LoadMapMessage, SaveMapMessage},
        resources::ActiveMap,
    },
    mission::{
        io::operations::read_mission_data, messages::SaveMissionMessage, resources::ActiveMission,
    },
    props::messages::LoadPropsMessage,
};

pub fn load_editor(
    mut load_editor_reader: MessageReader<LoadEditorMessage>,
    mut load_map_writer: MessageWriter<LoadMapMessage>,
    mut load_props_writer: MessageWriter<LoadPropsMessage>,
    mut next_state: ResMut<NextState<AppState>>,
    mut commands: Commands,
) {
    for message in load_editor_reader.read() {
        let Ok(mission) = read_mission_data(&message.id) else {
            return;
        };

        load_map_writer.write(LoadMapMessage { id: mission.map_id });
        load_props_writer.write(LoadPropsMessage { id: mission.map_id });

        commands.insert_resource(ActiveMission { mission });
        commands.insert_resource(EditorTool::None);
        commands.insert_resource(EditorSettings {
            snap_to_grid: true,
            tile_aligned: false,
        });

        commands.spawn((
            PlacementPreview {
                size: Vec2::default(),
            },
            Sprite::default(),
            Transform::default(),
            Visibility::Hidden,
        ));

        next_state.set(AppState::Editor);
    }
}

pub fn exit_editor(
    mut commands: Commands,
    map_query: Query<Entity, With<MissionMapChunk>>,
    placement_preview_query: Query<Entity, With<PlacementPreview>>,
) {
    // despawning map
    for entity in map_query.iter() {
        commands.entity(entity).despawn();
    }

    // despawning preview
    if let Ok(entity) = placement_preview_query.single() {
        commands.entity(entity).despawn();
    }

    //removing editor resources
    commands.remove_resource::<ActiveMission>();
    commands.remove_resource::<EditorTool>();
    commands.remove_resource::<EditorSettings>();
}

const TILE_INDEX: u32 = 1;

pub fn update_map_bounds(
    mut active_map: ResMut<ActiveMap>,
    mut map_bounds_reader: MessageReader<UpdateMapBoundsMessage>,
) {
    for message in map_bounds_reader.read() {
        let map_info = active_map.map.clone();
        match message.action {
            MapBoundOperationEnum::Shrink(amount) => match message.direction {
                MapBoundDirectionEnum::North => {
                    active_map.map.tiles.remove(0);
                    active_map.map.bounds.north -= amount
                }
                MapBoundDirectionEnum::East => {
                    active_map.map.tiles =
                        row_operations(RowOperation::EastShrink, active_map.map.tiles.clone());
                    active_map.map.bounds.east -= amount
                }
                MapBoundDirectionEnum::South => {
                    active_map.map.tiles.pop();
                    active_map.map.bounds.south -= amount
                }
                MapBoundDirectionEnum::West => {
                    active_map.map.tiles =
                        row_operations(RowOperation::WestShrink, active_map.map.tiles.clone());
                    active_map.map.bounds.west -= amount
                }
            },
            MapBoundOperationEnum::Expand(amount) => match message.direction {
                MapBoundDirectionEnum::North => {
                    active_map
                        .map
                        .tiles
                        .insert(0, vec![TILE_INDEX; map_info.tiles[0].len()]);
                    active_map.map.bounds.north += amount
                }
                MapBoundDirectionEnum::East => {
                    active_map.map.tiles =
                        row_operations(RowOperation::EastExpand, active_map.map.tiles.clone());
                    active_map.map.bounds.east += amount
                }
                MapBoundDirectionEnum::South => {
                    active_map
                        .map
                        .tiles
                        .push(vec![TILE_INDEX; map_info.tiles[0].len()]);
                    active_map.map.bounds.south += amount
                }
                MapBoundDirectionEnum::West => {
                    active_map.map.tiles =
                        row_operations(RowOperation::WestExpand, active_map.map.tiles.clone());
                    active_map.map.bounds.west += amount
                }
            },
        }
    }
}

enum RowOperation {
    WestExpand,
    EastExpand,
    WestShrink,
    EastShrink,
}

fn row_operations(operation: RowOperation, mut tiles: Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    for row in tiles.iter_mut() {
        match operation {
            RowOperation::WestExpand => row.insert(0, TILE_INDEX),
            RowOperation::EastExpand => row.push(TILE_INDEX),
            RowOperation::WestShrink => {
                row.remove(0);
            }
            RowOperation::EastShrink => {
                row.pop();
            }
        }
    }
    tiles
}

pub fn save_editor_changes(
    mut save_map_writer: MessageWriter<SaveMapMessage>,
    mut save_mission_writer: MessageWriter<SaveMissionMessage>,
) {
    save_map_writer.write(SaveMapMessage);
    save_mission_writer.write(SaveMissionMessage);
}

pub fn render_gizmos(active_mission: Res<ActiveMission>, mut gizmos: Gizmos) {
    let spawn_position = active_mission.mission.player_spawn;

    gizmos.rounded_rect_2d(
        Isometry2d::from_xy(spawn_position.x, spawn_position.y),
        Vec2::splat(50.),
        Color::linear_rgb(1., 0.4, 0.4),
    );
}

const CAMERA_SPEED: f32 = 100.;

pub fn editor_camera_controller(
    keys: Res<ButtonInput<KeyCode>>,
    mut camera_query: Query<(&mut Transform, &mut Projection), With<Camera>>,
    time: Res<Time>,
    mut evr_scroll: MessageReader<MouseWheel>,
) {
    let Ok((mut camera_transform, mut projection)) = camera_query.single_mut() else {
        error!("no camera found");
        return;
    };
    let mut direction = Vec2::default();
    let mut speed_multiplier: f32 = 1.;

    if keys.pressed(KeyCode::KeyA) {
        direction.x += -1.;
    }
    if keys.pressed(KeyCode::KeyD) {
        direction.x += 1.;
    }
    if keys.pressed(KeyCode::KeyW) {
        direction.y += 1.;
    }

    if keys.pressed(KeyCode::KeyS) {
        direction.y += -1.;
    }

    //TEMPORARY - for now we just keep it simple and can increase speed with shift
    if keys.pressed(KeyCode::ShiftLeft) {
        speed_multiplier = 2.
    }

    let displacement = direction * (CAMERA_SPEED * speed_multiplier) * time.delta_secs();

    camera_transform.translation += displacement.extend(0.0);

    //FIXME - this is currently a funky way of doing it, i'll see later to clean it up
    for ev in evr_scroll.read() {
        match ev.unit {
            MouseScrollUnit::Line => {
                if let Projection::Orthographic(ref mut ortho) = *projection {
                    ortho.scale -= ev.y * 0.1;
                    ortho.scale = ortho.scale.clamp(0.1, 10.0);
                }
            }
            MouseScrollUnit::Pixel => {
                // do nothing
            }
        }
    }
}
