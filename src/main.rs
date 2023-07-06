use bevy::prelude::*;
use bevy_ggrs::ggrs::{Config, SessionBuilder};
use bevy_ggrs::GGRSPlugin;
use bevy_matchbox::prelude::*;
use input::input;
use camera::CameraPlugin;
use network::{GgrsConfig, start_socket, wait_for_players};

mod arena;
mod player;
mod camera;
mod input;
mod network;

const FPS: usize = 60;
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
        .add_plugin(CameraPlugin)
        .add_plugin(NetworkPlugin)
        .run();
}


// Time Energy
pub struct Ether(f64);

// Magnetic field
