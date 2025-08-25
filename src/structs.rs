use bevy::prelude::*;

#[derive(Component)]
pub struct FpsText;

#[derive(Resource)]
pub struct FpsCache {
    pub last: i32,
}

#[derive(Component)]
pub struct Player;
