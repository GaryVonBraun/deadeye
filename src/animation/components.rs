use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct SpriteAnimator {
    pub current_clip: String,
    pub frame_timer: Timer,
    pub current_frame: usize,
    pub def_id: String,
}
