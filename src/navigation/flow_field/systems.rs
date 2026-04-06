use std::{collections::VecDeque, u32};

use bevy::prelude::*;

use crate::{
    map::{
        resources::ActiveMap,
        utility::{grid_to_world, world_to_grid},
    },
    navigation::{flow_field::components::FlowFieldTarget, resources::NavGrid},
};

pub fn build_flow_field(
    mut target_query: Query<(&mut FlowFieldTarget, &Transform)>,
    nav_grid: Res<NavGrid>,
    active_map: Res<ActiveMap>,
) {
    for (mut flow_field, transform) in target_query.iter_mut() {
        let current_tile = world_to_grid(
            transform.translation.truncate(),
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

        let mut directions_grid: Vec<Vec<Option<Vec2>>> =
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
                let mut best_direction = Vec2::ZERO;

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
                        best_direction = Vec2::new(dx as f32, -dy as f32).normalize();
                    }
                }

                directions_grid[y as usize][x as usize] = Some(best_direction);
            }
        }

        flow_field.directions = directions_grid;
    }
}

pub fn flow_field_gizmos(
    target_query: Query<&FlowFieldTarget>,
    active_map: Res<ActiveMap>,
    mut gizmos: Gizmos,
) {
    for flow_field in target_query.iter() {
        // flowfield is empty if there is not been a calculated tile before
        if flow_field.last_calculated_tile == None {
            continue;
        }
        for y in 0..flow_field.directions.len() {
            for x in 0..flow_field.directions[y].len() {
                let Some(direction) = flow_field.directions[y][x] else {
                    continue;
                };

                let tile_center = grid_to_world(
                    x as i32,
                    y as i32,
                    active_map.tileset.tile_size,
                    &active_map.map.bounds,
                );

                let scale = 25.;

                gizmos.arrow_2d(tile_center, tile_center + direction * scale, Color::WHITE);
            }
        }
    }
}
