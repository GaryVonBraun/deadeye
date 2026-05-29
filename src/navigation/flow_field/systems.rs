use std::{collections::VecDeque, u32};

use bevy::{ecs::batching::BatchingStrategy, prelude::*};

use crate::{
    ai::components::{AiController, AiLocomotionIntent, AiMovementIntent},
    collision::components::Collision,
    combat::health::components::Dead,
    map::{
        resources::ActiveMap,
        utility::{grid_to_world, world_to_grid},
    },
    navigation::{
        components::NavigationTargetTile,
        flow_field::components::{FlowFieldNavigator, FlowFieldTarget},
        resources::NavGrid,
    },
};

pub fn build_flow_field(
    mut target_query: Query<(&mut FlowFieldTarget, &Transform, &Collision)>,
    nav_grid: Res<NavGrid>,
    active_map: Res<ActiveMap>,
) {
    for (mut flow_field, transform, collision) in target_query.iter_mut() {
        let current_tile = world_to_grid(
            transform.translation.truncate() + collision.offset,
            active_map.tileset.tile_size,
            &active_map.map.bounds,
        );

        // if the tile is the same as last calculated it does not need to update
        let needs_update = match flow_field.last_calculated_tile {
            Some(tile) => tile != current_tile,
            None => true,
        };

        if !needs_update {
            continue;
        }
        flow_field.last_calculated_tile = Some(current_tile);

        // checking if target is within bounds
        if current_tile.1 as usize > active_map.map.tiles.len()
            || current_tile.0 as usize > active_map.map.tiles[0].len()
        {
            continue;
        }

        // initialize cost grid
        let mut cost_grid: Vec<Vec<Option<u32>>> =
            vec![vec![None; nav_grid.width as usize]; nav_grid.height as usize];

        // setting the target position
        cost_grid[current_tile.1 as usize][current_tile.0 as usize] = Some(0);

        let mut queue: VecDeque<(i32, i32)> = VecDeque::new();

        queue.push_back((current_tile.0, current_tile.1));

        let directions = [
            (-1, 0),
            (1, 0),
            (0, -1),
            (0, 1),
            (-1, -1),
            (-1, 1),
            (1, -1),
            (1, 1),
        ];

        while let Some((x, y)) = queue.pop_front() {
            for (dx, dy) in directions {
                // diagonal is added to tile position to create an offset
                let nx = x + dx;
                let ny = y + dy;

                // checking bounds
                if nx < 0 || ny < 0 || nx >= nav_grid.width as i32 || ny >= nav_grid.height as i32 {
                    continue;
                }

                // if tile is blocked or already assigned we skip
                if !nav_grid.cells[ny as usize][nx as usize]
                    || cost_grid[ny as usize][nx as usize] != None
                {
                    continue;
                }

                let is_diagonal = dx != 0 && dy != 0;
                if is_diagonal {
                    let ax = x + dx;
                    let ay = y;
                    let bx = x;
                    let by = y + dy;

                    // make sure diagonal tiles actually exist are not out of bounds
                    if ax < 0
                        || ax >= nav_grid.width as i32
                        || ay < 0
                        || ay >= nav_grid.height as i32
                        || bx < 0
                        || bx >= nav_grid.width as i32
                        || by < 0
                        || by >= nav_grid.height as i32
                    {
                        continue;
                    }

                    // check if they are passible
                    let passable_a = nav_grid.cells[y as usize][(x + dx) as usize];
                    let passable_b = nav_grid.cells[(y + dy) as usize][x as usize];
                    if !passable_a || !passable_b {
                        continue;
                    }
                }
                let current_cost = cost_grid[y as usize][x as usize].unwrap();
                if is_diagonal {
                    cost_grid[ny as usize][nx as usize] = Some(current_cost + 14);
                } else {
                    cost_grid[ny as usize][nx as usize] = Some(current_cost + 10);
                }
                queue.push_back((nx, ny));
            }
        }

        // info!("cost grid - {:?}", cost_grid);
        flow_field.costs = cost_grid;

        let mut waypoint_grid: Vec<Vec<Option<IVec2>>> =
            vec![vec![None; nav_grid.width as usize]; nav_grid.height as usize];

        for y in 0..nav_grid.height as i32 {
            for x in 0..nav_grid.width as i32 {
                if flow_field.costs[y as usize][x as usize] == None
                    || flow_field.costs[y as usize][x as usize] == Some(0)
                {
                    // none and 0 cannot be a direction
                    continue;
                }

                let mut cheapest_cost = u32::MAX;
                let mut best_tile = IVec2::ZERO;

                for (dx, dy) in directions {
                    let nx = x + dx;
                    let ny = y + dy;

                    // checking bounds
                    if nx < 0
                        || ny < 0
                        || nx >= nav_grid.width as i32
                        || ny >= nav_grid.height as i32
                    {
                        continue;
                    }

                    let is_diagonal = dx != 0 && dy != 0;
                    if is_diagonal {
                        // check if they are passible
                        let passable_a = nav_grid.cells[y as usize][(x + dx) as usize];
                        let passable_b = nav_grid.cells[(y + dy) as usize][x as usize];
                        if !passable_a || !passable_b {
                            continue;
                        }
                    }
                    let Some(tile_cost) = flow_field.costs[ny as usize][nx as usize] else {
                        continue;
                    };

                    if tile_cost < cheapest_cost {
                        cheapest_cost = tile_cost;
                        best_tile = IVec2 { x: nx, y: ny }
                    }
                }

                waypoint_grid[y as usize][x as usize] = Some(best_tile);
            }
        }

        flow_field.waypoint_grid = waypoint_grid;
    }
}

pub fn flow_field_navigation(
    mut ai_query: Query<
        (
            &AiController,
            &Transform,
            &mut NavigationTargetTile,
            &Collision,
        ),
        (Without<Dead>, With<FlowFieldNavigator>),
    >,
    target_query: Query<(Entity, &FlowFieldTarget)>,
    active_map: Res<ActiveMap>,
) {
    // Pre-compute tile bounds once so each parallel task doesn't recompute them.
    let tile_cols = active_map.map.tiles.first().map_or(0, |row| row.len());
    let tile_rows = active_map.map.tiles.len();

    // Collect flow field targets — typically just 1 (the player).
    // Vec<(Entity, &FlowFieldTarget)> is Send because FlowFieldTarget: Sync.
    let targets: Vec<(Entity, &FlowFieldTarget)> = target_query.iter().collect();

    ai_query
        .par_iter_mut()
        .batching_strategy(BatchingStrategy::fixed(100000))
        .for_each(
            |(controller, ai_transform, mut navigation_target, collision)| {
                let offset_position = ai_transform.translation.truncate() + collision.offset;

                //FIXME - setting the the target tile to ZERO every time is not needed i think
                let AiLocomotionIntent::Chase(target) = controller.black_board.locomotion_intent
                else {
                    navigation_target.0 = None;
                    return;
                };

                // Linear scan — fine since there are very few targets (usually just 1)
                let target_flow_field = match targets.iter().find(|(e, _)| *e == target) {
                    Some((_, ff)) => *ff,
                    None => return,
                };

                // this ensures the flow field is calculated first
                if target_flow_field.last_calculated_tile.is_none() {
                    return;
                }

                let current_tile = world_to_grid(
                    offset_position,
                    active_map.tileset.tile_size,
                    &active_map.map.bounds,
                );

                // this ensures that the entity is on the grid, it would crash if it was not
                if current_tile.0 as usize >= tile_cols || current_tile.1 as usize >= tile_rows {
                    return;
                }

                // get the direction of current tile position
                //FIXME - currently if the entity is on the target tile it does not know what to do, potential solution is to fallback on direct steering
                let Some(target_tile) = target_flow_field.waypoint_grid[current_tile.1 as usize]
                    [current_tile.0 as usize]
                else {
                    // error!("Could not find direction");
                    return;
                };

                navigation_target.0 = Some(target_tile);
            },
        );
}

pub fn flow_field_gizmos(
    target_query: Query<&FlowFieldTarget>,
    active_map: Res<ActiveMap>,
    mut gizmos: Gizmos,
) {
    //FIXME - the old gizmo's were based on a direction vector, currently model is now a tile position
    for flow_field in target_query.iter() {
        // flowfield is empty if there is not been a calculated tile before
        if flow_field.last_calculated_tile == None {
            continue;
        }
        for y in 0..flow_field.waypoint_grid.len() {
            for x in 0..flow_field.waypoint_grid[y].len() {
                let Some(direction) = flow_field.waypoint_grid[y][x] else {
                    continue;
                };

                let tile_center = grid_to_world(
                    x as i32,
                    y as i32,
                    active_map.tileset.tile_size,
                    &active_map.map.bounds,
                );

                let scale = 25.;

                // gizmos.arrow_2d(tile_center, tile_center + direction * scale, Color::WHITE);
            }
        }
    }
}
