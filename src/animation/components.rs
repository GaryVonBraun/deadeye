use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct SpriteAnimator {
    pub current_clip: String,
    pub frame_timer: Timer,
    pub current_frame: usize,
    pub def_id: String,
    pub clip_dirty: bool,
    pub flip_x: bool,
}

#[derive(Component, Debug)]
pub struct AnimationFinished;
