use bevy::prelude::*;
use bevy_ggrs::ggrs::PlayerHandle;

#[derive(Component)]
pub struct Player {
    pub handle: PlayerHandle,
}

#[derive(Component, Reflect, Default)]
pub struct Velocity(pub Vec3);

impl Player {
    pub fn new() -> Self {
        Self { handle: 0  }
    }
}


