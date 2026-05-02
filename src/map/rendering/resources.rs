use bevy::prelude::*;

#[derive(Resource, Clone)]
pub enum TilesetRenderState {
    NeedsLoading,
    Loading(Handle<Image>),
    NeedsPadding(Handle<Image>),
    Ready(Handle<Image>),
}
