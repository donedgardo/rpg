pub struct Player;

impl Player {
    pub fn new() -> Self {
        Self
    }
}
use bevy::prelude::*;
use bevy_ggrs::ggrs::PlayerHandle;

pub struct Player {
    pub handle: PlayerHandle,
}
