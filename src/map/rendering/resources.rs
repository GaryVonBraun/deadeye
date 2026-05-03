use bevy::prelude::*;

#[derive(Debug, Resource, Clone)]
pub enum TilesetRenderState {
    Loading(Handle<Image>),
    Ready(Handle<Image>),
    Cashed(Handle<Image>),
}
