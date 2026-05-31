use bevy::prelude::*;
use bevy_egui::EguiPrimaryContextPass;

use crate::{
    debug::{resources::DebugOptions, systems::*},
    map::resources::ActiveMap,
    navigation::resources::NavGrid,
};

pub mod components;
mod resources;
mod systems;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource::<DebugOptions>(DebugOptions::default());
        app.add_systems(EguiPrimaryContextPass, display_debug_menu);
        app.add_systems(
            Update,
            (
                debug_movement_controller,
                debug_vision_gizmo.run_if(|opts: Res<DebugOptions>| opts.vision),
                debug_visible_entities_gizmo.run_if(|opts: Res<DebugOptions>| opts.visible_actors),
                debug_nearest_visible_hostile_gizmo
                    .run_if(|opts: Res<DebugOptions>| opts.nearest_visible_hostile),
                debug_hitbox_gizmo.run_if(|opts: Res<DebugOptions>| opts.hit_box),
                debug_hurtbox_gizmo.run_if(|opts: Res<DebugOptions>| opts.hurt_box),
                debug_collision_gizmo.run_if(|opts: Res<DebugOptions>| opts.collision),
                astar_gizmos
                    .run_if(resource_exists::<NavGrid>)
                    .run_if(|opts: Res<DebugOptions>| opts.astar_paths),
            ),
        );
    }
}
