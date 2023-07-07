use bevy::prelude::*;
use bevy::app::AppExit;
use bevy_ggrs::ggrs::{Config, SessionBuilder};
use bevy_ggrs::GGRSPlugin;
use bevy_matchbox::prelude::*;
use input::input;
use camera::CameraPlugin;
use network::{GgrsConfig};
use network::NetworkPlugin;
use menu::MenuPlugin;

mod arena;
mod player;
mod camera;
mod input;
mod network;
mod menu;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum AppState {
    Menu,
    Online,
}

fn main() {
    let mut app = App::new();
    app
        .add_plugins(DefaultPlugins)
        .add_plugin(CameraPlugin)
        .add_plugin(NetworkPlugin)
        .add_plugin(MenuPlugin)
        .add_state(AppState::Menu)
        .add_system_set(SystemSet::on_enter(AppState::Online).with_system(network::start_socket.system()))
        .run();
}


// Time Energy
pub struct Ether(f64);
// Magnetic field
