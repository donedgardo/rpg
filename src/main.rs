use bevy::prelude::*;
use bevy::app::AppExit;
use bevy_ggrs::ggrs::{Config, SessionBuilder};
use bevy_ggrs::GGRSPlugin;
use bevy_matchbox::prelude::*;
use app_state::AppState;
use input::input;
use camera::CameraPlugin;
use network::GgrsConfig;
use network::NetworkPlugin;
use menu::MenuPlugin;

mod arena;
mod player;
mod camera;
mod input;
mod network;
mod menu;
mod app_state;

fn main() {
    let mut app = App::new();
    app
        .add_state::<AppState>()
        .add_plugins(DefaultPlugins)
        .add_plugin(CameraPlugin)
        .add_plugin(NetworkPlugin)
        .add_plugin(MenuPlugin)
        .run();
}


// Time Energy
pub struct Ether(f64);
// Magnetic field
