use bevy::prelude::*;

#[derive(Component)]
pub struct Actor;

#[derive(Bundle)]
pub struct CoreActorBundle{
    pub transform: Transform,
    pub actor:Actor
}
