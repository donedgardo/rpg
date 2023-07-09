use bevy::prelude::*;
use bevy_ggrs::ggrs::PlayerHandle;

#[derive(Component)]
pub struct Player {
    pub handle: PlayerHandle,
}

impl Player {
    pub fn new() -> Self {
        Self { handle: 0  }
    }
}


