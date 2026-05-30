use std::{cmp::Ordering, collections::BinaryHeap};

use bevy::prelude::*;

use crate::{
    collision::components::Collision,
    map::{resources::ActiveMap, utility::world_to_grid},
    navigation::{
        astar::components::AStarPath, components::NavigationTargetTile, resources::NavGrid,
    },
};

#[derive(Eq, PartialEq, PartialOrd)]
struct AStarNode {
    position: (i32, i32),
    g_cost: u32,
    f_score: u32,
}

impl Ord for AStarNode {
    fn cmp(&self, other: &Self) -> Ordering {
        // its being ordered in reverse due
        other.f_score.cmp(&self.f_score)
        //NOTE - currently only using f_score meaning it could have a tie when ordering
    }
}

fn astar_heuristic(start: IVec2, target: IVec2) -> u32 {
    // get difference on both axis
    let dx = start.x.abs_diff(target.x);
    let dy = start.y.abs_diff(target.y);

    let diagonal = dx.min(dy);
    let straight = dx.max(dy) - diagonal;

    diagonal * 14 + straight * 10
}

pub fn calculate_astar_path(
    mut astar_query: Query<(&mut AStarPath, &Transform, &Collision)>,
    nav_grid: Res<NavGrid>,
    active_map: Res<ActiveMap>,
) {
    for (mut astar, transform, collision) in astar_query.iter_mut() {
        let start = world_to_grid(
            transform.translation.truncate() + collision.offset,
            active_map.tileset.tile_size,
            &active_map.map.bounds,
        );

        // no target -> skip
        let Some(target) = astar.target else {
            continue;
        };

        // don't recalc same target
        if astar.calculated_target == astar.target {
            continue;
        }

        astar.calculated_target = astar.target;

        // bounds check (fixed logic)
        if start.x < 0
            || start.y < 0
            || start.x >= nav_grid.width as i32
            || start.y >= nav_grid.height as i32
        {
            continue;
        }

        // a* data structures

        let mut cost_grid: Vec<Vec<Option<u32>>> =
            vec![vec![None; nav_grid.width as usize]; nav_grid.height as usize];

        let mut came_from: Vec<Vec<Option<IVec2>>> =
            vec![vec![None; nav_grid.width as usize]; nav_grid.height as usize];

        let mut queue: BinaryHeap<AStarNode> = BinaryHeap::new();

        cost_grid[start.y as usize][start.x as usize] = Some(0);

        queue.push(AStarNode {
            position: (start.x, start.y),
            g_cost: 0,
            f_score: astar_heuristic(start, target),
        });

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

        // *star search

        while let Some(current_node) = queue.pop() {
            let (x, y) = current_node.position;
            let current = IVec2::new(x, y);

            // goal reached -> stop
            if current == target {
                break;
            }

            for (dx, dy) in directions {
                let nx = x + dx;
                let ny = y + dy;

                // bounds
                if nx < 0 || ny < 0 || nx >= nav_grid.width as i32 || ny >= nav_grid.height as i32 {
                    continue;
                }

                // walkable check
                if !nav_grid.cells[ny as usize][nx as usize] {
                    continue;
                }

                let is_diagonal = dx != 0 && dy != 0;

                if is_diagonal {
                    let passable_a = nav_grid.cells[y as usize][(nx) as usize];
                    let passable_b = nav_grid.cells[(ny) as usize][x as usize];

                    if !passable_a || !passable_b {
                        continue;
                    }
                }

                let step_cost = if is_diagonal { 14 } else { 10 };

                let new_cost = current_node.g_cost + step_cost;

                // if we've seen a better path, skip
                if let Some(old_cost) = cost_grid[ny as usize][nx as usize] {
                    if new_cost >= old_cost {
                        continue;
                    }
                }

                cost_grid[ny as usize][nx as usize] = Some(new_cost);
                came_from[ny as usize][nx as usize] = Some(current);

                let neighbor = IVec2::new(nx, ny);
                let h = astar_heuristic(neighbor, target);

                queue.push(AStarNode {
                    position: (nx, ny),
                    g_cost: new_cost,
                    f_score: new_cost + h,
                });
            }
        }

        // path reconstruction

        let mut path = Vec::new();
        let mut current = target;

        while current != start {
            path.push(current);

            let Some(prev) = came_from[current.y as usize][current.x as usize] else {
                // no path found
                break;
            };

            current = prev;
        }

        path.reverse();
        astar.path = path;
    }
}

pub fn astar_navigation(
    mut astar_query: Query<(
        &mut AStarPath,
        &Transform,
        &Collision,
        &mut NavigationTargetTile,
    )>,
    active_map: Res<ActiveMap>,
) {
    for (mut astar_path, transform, collision, mut navigation_target) in astar_query.iter_mut() {
        if astar_path.path.is_empty() {
            continue;
        }

        let offset_position = transform.translation.truncate() + collision.offset;
        let current_tile = world_to_grid(
            offset_position,
            active_map.tileset.tile_size,
            &active_map.map.bounds,
        );

        if let Some(target) = astar_path.target {
            if current_tile == target {
                navigation_target.0 = None;
                continue;
            }
        }

        if let Some(target) = navigation_target.0 {
            if current_tile == target && astar_path.current_index + 1 < astar_path.path.len() {
                astar_path.current_index += 1;
            }
        };

        navigation_target.0 = Some(astar_path.path[astar_path.current_index]);
    }
}
