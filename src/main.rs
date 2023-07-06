use bevy::prelude::*;
use bevy_ggrs::ggrs::{Config, SessionBuilder};
use bevy_ggrs::GGRSPlugin;
use bevy_matchbox::prelude::*;
use input::input;
use camera::CameraPlugin;
use network::{GgrsConfig};
use network::NetworkPlugin;

mod arena;
mod player;
mod camera;
mod input;
mod network;

const FPS: usize = 60;
fn main() {
    let mut app = App::new();
    network::integrate_ggrs_plugin(&mut app);
    app
        .add_plugins(DefaultPlugins)
        .add_plugin(CameraPlugin)
        .add_plugin(NetworkPlugin)
        .run();
}


// Time Energy
pub struct Ether(f64);

// Magnetic field
