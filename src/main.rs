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
mod cleanup_ui;

fn main() {
    let mut app = App::new();
    app
        .add_state(AppState::Menu)
        .add_plugins(DefaultPlugins)
        .add_plugin(CameraPlugin)
        .add_plugin(NetworkPlugin)
        .add_plugin(MenuPlugin)
        .add_system_set(SystemSet::on_enter(AppState::WaitingForPlayers).with_system(show_waiting_text.system()))
        .run();
}

fn show_waiting_text(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("Roboto/Roboto-Regular.ttf");
    commands.spawn(TextBundle {
        text: Text {
            value: "Waiting for players...".to_string(),
            font: font,
            style: TextStyle {
                font_size: 50.0,
                color: Color::WHITE,
                ..Default::default()
            },
        },
        ..Default::default()
    });
}

// Time Energy
pub struct Ether(f64);
// Magnetic field
