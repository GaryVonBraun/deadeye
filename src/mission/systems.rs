use bevy::{prelude::*, state::commands};
use rand::RngExt;
use uuid::Uuid;

use crate::{
    actor::{components::Zombie, messages::SpawnActorMessage},
    campaign::resources::Campaign,
    combat::health::components::Dead,
    core::states::AppState,
    editor::messages::LoadEditorMessage,
    map::{
        io::{
            operations::write_map,
            types::{MapBounds, MissionMap},
        },
        messages::LoadMapMessage,
        resources::ActiveMap,
    },
    mission::{
        io::operations::{
            read_mission_data, remove_mission_file, update_mission_data, write_mission,
        },
        messages::*,
        resources::{ActiveMission, GameOverData, Mission, WaveSpawnerState},
    },
    navigation::messages::BuildNavGridMessage,
    player::components::Player,
    props::messages::LoadPropsMessage,
    ui::missions_menu::messages::RefreshMissionListMessage,
};

pub fn test_mission(
    mut test_mission_reader: MessageReader<TestMissionMessage>,
    mut next_state: ResMut<NextState<AppState>>,
    mut load_map_writer: MessageWriter<LoadMapMessage>,
    mut load_props_writer: MessageWriter<LoadPropsMessage>,
    mut spawn_actor_writer: MessageWriter<SpawnActorMessage>,
    mut build_nav_grid_writer: MessageWriter<BuildNavGridMessage>,
    mut commands: Commands,
) {
    for message in test_mission_reader.read() {
        let Ok(mission) = read_mission_data(&message.id) else {
            return;
        };
        info!("load mission: {:?}", message.id);

        info!("found mission called: {:?}", mission.name);

        // set state to game
        next_state.set(AppState::InGame);

        //set active map
        load_map_writer.write(LoadMapMessage { id: mission.map_id });

        load_props_writer.write(LoadPropsMessage { id: mission.map_id });

        build_nav_grid_writer.write(BuildNavGridMessage { id: mission.map_id });

        // is a mission has waves we insert the spawner state here
        if !mission.waves.is_empty() {
            commands.insert_resource(WaveSpawnerState {
                current_wave: 0,
                wave_zombies_spawned: 0,
                spawn_timer: Timer::from_seconds(
                    1.0 / mission.waves[0].spawn_per_second,
                    TimerMode::Repeating,
                ),
                wave_delay_timer: None,
                finished: false,
            });
        }

        spawn_actor_writer.write(SpawnActorMessage {
            id: "player".to_string(),
            position: mission.player_spawn,
            weapons: vec!["ak".to_string()],
        });
        commands.insert_resource(ActiveMission { mission: mission });

        //TEMPORARY - for now we spawn a zombie and npc like this just to be able to test it
        // for i in 0..50 {
        //     for j in 0..50 {
        //         spawn_actor_writer.write(SpawnActorMessage {
        //             id: "zombie".to_string(),
        //             position: Vec2 {
        //                 x: 800. + 35. * i as f32,
        //                 y: -500. + 35. * j as f32,
        //             },
        //         });
        //     }
        // }

        for y in 0..1 {
            for x in 0..1 {
                spawn_actor_writer.write(SpawnActorMessage {
                    id: "npc".to_string(),
                    position: Vec2 {
                        x: x as f32 * 5.,
                        y: y as f32 * 5.,
                    },
                    weapons: vec!["ak".to_string()],
                });
            }
        }
    }
}

pub fn load_mission(
    mut load_mission_reader: MessageReader<LoadMissionMessage>,
    mut next_state: ResMut<NextState<AppState>>,
    mut load_map_writer: MessageWriter<LoadMapMessage>,
    mut load_props_writer: MessageWriter<LoadPropsMessage>,
    mut spawn_actor_writer: MessageWriter<SpawnActorMessage>,
    mut build_nav_grid_writer: MessageWriter<BuildNavGridMessage>,
    mut commands: Commands,
    campaign: Res<Campaign>,
) {
    for message in load_mission_reader.read() {
        let Ok(mission) = read_mission_data(&message.id) else {
            return;
        };
        info!("load mission: {:?}", message.id);

        info!("found mission called: {:?}", mission.name);

        // set state to game
        next_state.set(AppState::InGame);

        //set active map
        load_map_writer.write(LoadMapMessage { id: mission.map_id });

        load_props_writer.write(LoadPropsMessage { id: mission.map_id });

        build_nav_grid_writer.write(BuildNavGridMessage { id: mission.map_id });

        // is a mission has waves we insert the spawner state here
        if !mission.waves.is_empty() {
            commands.insert_resource(WaveSpawnerState {
                current_wave: 0,
                wave_zombies_spawned: 0,
                spawn_timer: Timer::from_seconds(
                    1.0 / mission.waves[0].spawn_per_second,
                    TimerMode::Repeating,
                ),
                wave_delay_timer: None,
                finished: false,
            });
        }

        spawn_actor_writer.write(SpawnActorMessage {
            id: "player".to_string(),
            position: mission.player_spawn,
            weapons: vec!["ak".to_string()],
        });
        commands.insert_resource(ActiveMission { mission: mission });

        //TEMPORARY - for now we spawn a zombie and npc like this just to be able to test it
        // for i in 0..50 {
        //     for j in 0..50 {
        //         spawn_actor_writer.write(SpawnActorMessage {
        //             id: "zombie".to_string(),
        //             position: Vec2 {
        //                 x: 800. + 35. * i as f32,
        //                 y: -500. + 35. * j as f32,
        //             },
        //         });
        //     }
        // }

        for x in 0..campaign.squad.len() {
            spawn_actor_writer.write(SpawnActorMessage {
                id: "npc".to_string(),
                position: Vec2 {
                    x: x as f32 * 5.,
                    y: 0. as f32 * 5.,
                },
                weapons: vec!["ak".to_string()],
            });
        }
    }
}

// crud systems
pub fn save_mission(active_mission: Res<ActiveMission>) {
    info!("{:?}", active_mission);
    update_mission_data(&active_mission.mission);
}

pub fn create_new_mission(mut load_editor_writer: MessageWriter<LoadEditorMessage>) {
    info!("creating mission");

    //NOTE - i am not really a fan of this but this is the simplest way for now
    let map = create_map_for_mission();

    let mut rng = rand::rng();

    let mission = Mission {
        id: Uuid::new_v4(),
        name: format!("test mission {:?}", rng.random_range(1..1000)).to_string(),
        map_id: map.id,
        player_spawn: Vec2::splat(0.),
        spawn_inset: 1,
        waves: vec![],
    };

    // write the new mission to drive
    write_mission(&mission);

    load_editor_writer.write(LoadEditorMessage { id: mission.id });
}

fn create_map_for_mission() -> MissionMap {
    let raw_matrix: Vec<Vec<u32>> = vec![vec![5, 5], vec![5, 5]];

    let mut rng = rand::rng();

    let map = MissionMap {
        name: format!("test map {:?}", rng.random_range(1..1000)).to_string(),
        id: Uuid::new_v4(),
        tiles: raw_matrix,
        tileset_name: "Background_Green_TileSet".to_string(),
        bounds: MapBounds::default(),
        placed_props: vec![],
    };
    write_map(&map);
    map
}

pub fn delete_mission(
    mut delete_mission_reader: MessageReader<DeleteMissionMessage>,
    mut refresh_mission_writer: MessageWriter<RefreshMissionListMessage>,
) {
    for message in delete_mission_reader.read() {
        remove_mission_file(message.id);
    }
    refresh_mission_writer.write(RefreshMissionListMessage);
}

pub fn edit_mission(
    mut edit_mission_reader: MessageReader<EditMissionMessage>,
    mut load_editor_writer: MessageWriter<LoadEditorMessage>,
) {
    for mission in edit_mission_reader.read() {
        load_editor_writer.write(LoadEditorMessage { id: mission.id });
    }
}

pub fn wave_spawner_gizmo(
    active_map: Res<ActiveMap>,
    active_mission: Res<ActiveMission>,
    mut gizmos: Gizmos,
) {
    let bounds: &MapBounds = &active_map.map.bounds;

    let strips = calculate_spawn_strips(
        bounds,
        active_map.tileset.tile_size,
        active_mission.mission.spawn_inset,
    );

    for spawner_strip in strips {
        gizmos.rect_2d(
            spawner_strip.center,
            spawner_strip.size,
            Color::linear_rgb(0.5, 0., 0.),
        );
    }
}

pub struct SpawnStrip {
    pub center: Vec2,
    pub size: Vec2,
}

fn calculate_spawn_strips(bounds: &MapBounds, tile_size: f32, inset_tiles: u32) -> Vec<SpawnStrip> {
    let inset = inset_tiles as f32 * tile_size;
    let map_w = (bounds.east + bounds.west) as f32 * tile_size;
    let map_h = (bounds.north + bounds.south) as f32 * tile_size;
    let mut strips: Vec<SpawnStrip> = vec![];
    let center_x = (bounds.east as f32 - bounds.west as f32) * tile_size / 2.;
    let center_y = (bounds.north as f32 - bounds.south as f32) * tile_size / 2.;

    // north
    strips.push(SpawnStrip {
        center: Vec2 {
            x: center_x,
            y: (bounds.north as f32 * tile_size) - inset / 2.,
        },
        size: Vec2 { x: map_w, y: inset },
    });

    // south
    strips.push(SpawnStrip {
        center: Vec2 {
            x: center_x,
            y: -(bounds.south as f32 * tile_size) + inset / 2.,
        },
        size: Vec2 { x: map_w, y: inset },
    });

    // east
    strips.push(SpawnStrip {
        center: Vec2 {
            x: (bounds.east as f32 * tile_size) - inset / 2.,
            y: center_y,
        },
        size: Vec2 {
            x: inset,
            y: map_h - inset * 2.,
        },
    });

    // west
    strips.push(SpawnStrip {
        center: Vec2 {
            x: -(bounds.west as f32 * tile_size) + inset / 2.,
            y: center_y,
        },
        size: Vec2 {
            x: inset,
            y: map_h - inset * 2.,
        },
    });

    strips
}

pub fn wave_spawner(
    mut timer: Local<f32>,
    time: Res<Time>,
    active_map: Res<ActiveMap>,
    active_mission: Res<ActiveMission>,
    mut spawn_actor_writer: MessageWriter<SpawnActorMessage>,
    mut state: ResMut<WaveSpawnerState>,
) {
    // if waves are all finished just return
    if state.finished {
        return;
    }

    *timer += time.delta_secs();

    // get current wave definition
    let Some(current_wave) = active_mission.mission.waves.get(state.current_wave) else {
        error!("No wave definition found needed for wave spawner");
        return;
    };

    // if at the last wave and all zombies are spawned the waves are finished
    if active_mission
        .mission
        .waves
        .get(state.current_wave + 1)
        .is_none()
        && state.wave_zombies_spawned == current_wave.zombie_count
    {
        state.finished = true;
        info!("All waves finished");
        return;
    }

    state.spawn_timer.tick(time.delta());
    let times_fired = state.spawn_timer.times_finished_this_tick();

    for _ in 0..times_fired {
        if state.wave_zombies_spawned >= current_wave.zombie_count {
            if let Some(next_wave) = active_mission.mission.waves.get(state.current_wave + 1)
                && state.wave_zombies_spawned == current_wave.zombie_count
            {
                state.current_wave += 1;
                state.wave_zombies_spawned = 0;
                state.spawn_timer =
                    Timer::from_seconds(1.0 / next_wave.spawn_per_second, TimerMode::Repeating);
                return;
            }
            break;
        }
        let mut rng = rand::rng();
        let bounds: &MapBounds = &active_map.map.bounds;

        let strips = calculate_spawn_strips(
            bounds,
            active_map.tileset.tile_size,
            active_mission.mission.spawn_inset,
        );

        let strip = &strips[rng.random_range(0..strips.len())];

        let pos = Vec2::new(
            strip.center.x + rng.random_range(-strip.size.x / 2. ..strip.size.x / 2.),
            strip.center.y + rng.random_range(-strip.size.y / 2. ..strip.size.y / 2.),
        );

        spawn_actor_writer.write(SpawnActorMessage {
            id: "zombie".to_string(),
            position: pos,
            weapons: vec![],
        });
        state.wave_zombies_spawned += 1;
    }
}

pub fn player_death_check(
    player_query: Query<&Dead, With<Player>>,
    mut timer: Local<Option<Timer>>,
    time: Res<Time>,
    mut next_state: ResMut<NextState<AppState>>,
    mut commands: Commands,
    active_mission: Res<ActiveMission>,
) {
    if player_query.is_empty() {
        *timer = None;
        return;
    }

    let timer = timer.get_or_insert_with(|| Timer::from_seconds(2.5, TimerMode::Once));
    timer.tick(time.delta());

    if timer.just_finished() {
        commands.insert_resource(GameOverData {
            mission_id: active_mission.mission.id,
        });
        next_state.set(AppState::GameOver);
    }
}

pub fn check_victory_condition(
    state: Res<WaveSpawnerState>,
    zombie_query: Query<Entity, With<Zombie>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if state.finished && zombie_query.is_empty() {
        next_state.set(AppState::Victory);
    }
}
