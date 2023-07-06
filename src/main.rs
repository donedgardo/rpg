use bevy::prelude::*;
use bevy_ggrs::ggrs::{Config, SessionBuilder};
use bevy_ggrs::GGRSPlugin;
use bevy_matchbox::prelude::*;
use crate::input::input;

mod arena;
mod player;
mod camera;
mod input;

use camera::CameraPlugin;

struct GgrsConfig;

const FPS: usize = 60;
use crate::network::{start_socket, wait_for_players};
fn main() {

    let mut app = App::new();
    GGRSPlugin::<GgrsConfig>::new()
        // define frequency of rollback game logic update
        .with_update_frequency(FPS)
        // define system that returns inputs given a player handle, so GGRS can send the inputs around
        .with_input_system(input)
        .build(&mut app);
    app
        .add_plugins(DefaultPlugins)
        .add_startup_system(start_socket)
        .add_system(wait_for_players)
        .run();
}

// Time Energy
pub struct Ether(f64);

// Magnetic field
