use bevy::{math::ops::ceil, prelude::*};

use crate::map::resources::ActiveMap;

const CHUNK_SIZE: f32 = 16. * 64.;

pub fn render_map(active_map: Res<ActiveMap>) {
    info!("rendering map!");
}

pub fn rerender_map(active_map: Res<ActiveMap>) {
    info!("rerendering map!");

    info!(
        "chunks north{:?}",
        ceil(active_map.map.bounds.north as f32 / 16.)
    );
    info!(
        "chunks west{:?}",
        ceil(active_map.map.bounds.west as f32 / 16.)
    );
    info!(
        "chunks west{:?}",
        ceil(active_map.map.bounds.east as f32 / 16.)
    );
}

pub fn chunk_rendering_gizmos(active_map: Res<ActiveMap>, mut gizmos: Gizmos) {
    let total_y = ceil(active_map.map.bounds.north as f32 / 16.)
        + ceil(active_map.map.bounds.south as f32 / 16.);
    let total_x = ceil(active_map.map.bounds.west as f32 / 16.)
        + ceil(active_map.map.bounds.east as f32 / 16.);

    // gizmos.grid_2d(
    //     Vec2::splat(0.),
    //     UVec2 { x: 4, y: 4 },
    //     Vec2 { x: 64., y: 64. },
    //     Color::linear_rgb(1., 1.0, 1.0),
    // );

    // info!("total x:{:?}", total_x);

    for y in 0..total_y as i32 {
        for x in 0..total_x as i32 {
            gizmos.rect_2d(
                Vec2 {
                    x: (x as f32 - ceil(active_map.map.bounds.west as f32 / 16.)) * CHUNK_SIZE,
                    y: (y as f32 - ceil(active_map.map.bounds.south as f32 / 16.)) * CHUNK_SIZE,
                },
                Vec2::splat(CHUNK_SIZE),
                Color::linear_rgb(1., 0., 1.0),
            );
        }
    }
}
