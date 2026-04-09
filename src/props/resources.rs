use bevy::prelude::*;

use crate::{collision::components::Collision, props::io::types::PlacedProp};

#[derive(Debug, Resource)]
pub struct ActiveMapProps {
    pub props: Vec<PlacedProp>,
}

/// Static spatial hash for props. Built once on map load using a flat counting-sort
/// layout: two arrays (`flat` + `offsets`) replace the old HashMap<cell, Vec<...>>,
/// giving O(1) array-index lookups with no per-query hashing or heap indirection.
#[derive(Debug, Resource)]
pub struct PropSpatialHash {
    flat: Vec<(Vec2, Collision)>,
    offsets: Vec<u32>,
    pub cell_size: f32,
    min_cx: i32,
    min_cy: i32,
    grid_width: usize,
    grid_height: usize,
}

impl PropSpatialHash {
    /// Build from a flat list of `(cell_x, cell_y, world_pos, collision)` entries.
    pub fn from_entries(raw: Vec<(i32, i32, Vec2, Collision)>, cell_size: f32) -> Self {
        if raw.is_empty() {
            return PropSpatialHash {
                flat: vec![],
                offsets: vec![0],
                cell_size,
                min_cx: 0,
                min_cy: 0,
                grid_width: 0,
                grid_height: 0,
            };
        }

        let min_cx = raw.iter().map(|(cx, _, _, _)| *cx).min().unwrap();
        let min_cy = raw.iter().map(|(_, cy, _, _)| *cy).min().unwrap();
        let max_cx = raw.iter().map(|(cx, _, _, _)| *cx).max().unwrap();
        let max_cy = raw.iter().map(|(_, cy, _, _)| *cy).max().unwrap();

        let grid_width = (max_cx - min_cx + 1) as usize;
        let grid_height = (max_cy - min_cy + 1) as usize;
        let num_cells = grid_width * grid_height;

        // Pass 1: count entries per cell
        let mut counts = vec![0u32; num_cells];
        for (cx, cy, _, _) in &raw {
            let cell = (*cy - min_cy) as usize * grid_width + (*cx - min_cx) as usize;
            counts[cell] += 1;
        }

        // Pass 2: exclusive prefix sum → cell start offsets
        let mut offsets = vec![0u32; num_cells + 1];
        for i in 0..num_cells {
            offsets[i + 1] = offsets[i] + counts[i];
        }

        // Pass 3: fill flat array using per-cell cursors
        let total = offsets[num_cells] as usize;
        let mut flat = vec![(Vec2::ZERO, Collision::default()); total];
        let mut cursors = offsets[..num_cells].to_vec();
        for (cx, cy, pos, collision) in raw {
            let cell = (cy - min_cy) as usize * grid_width + (cx - min_cx) as usize;
            flat[cursors[cell] as usize] = (pos, collision);
            cursors[cell] += 1;
        }

        PropSpatialHash {
            flat,
            offsets,
            cell_size,
            min_cx,
            min_cy,
            grid_width,
            grid_height,
        }
    }

    /// Returns all prop entries whose bounding cell matches `(cx, cy)`.
    /// Out-of-bounds cells return an empty slice — no branching needed at call sites.
    pub fn neighbors(&self, cx: i32, cy: i32) -> &[(Vec2, Collision)] {
        let lx = cx - self.min_cx;
        let ly = cy - self.min_cy;
        if lx < 0
            || ly < 0
            || (lx as usize) >= self.grid_width
            || (ly as usize) >= self.grid_height
        {
            return &[];
        }
        let cell = ly as usize * self.grid_width + lx as usize;
        let start = self.offsets[cell] as usize;
        let end = self.offsets[cell + 1] as usize;
        &self.flat[start..end]
    }
}
