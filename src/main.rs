use bevy::prelude::*;
use bevy::app::AppExit;
use bevy::log::Level;
use bevy_ggrs::ggrs::{Config, SessionBuilder};
use bevy_ggrs::GGRSPlugin;
use bevy_matchbox::prelude::*;
use app_state::AppState;
use input::input;
use camera::CameraPlugin;
use network::GgrsConfig;
use network::NetworkPlugin;
use menu::MenuPlugin;
use bevy_editor_pls::prelude::*;
use bevy_rapier2d::plugin::RapierPhysicsPlugin;
use bevy_rapier2d::prelude::NoUserData;
use bevy_rapier2d::render::RapierDebugRenderPlugin;
use level::LevelPlugin;
use gamepad::GamepadPlugin;

mod arena;
mod player;
mod camera;
mod input;
mod network;
mod menu;
mod app_state;
mod cleanup_ui;
mod level;
mod gamepad;

fn main() {
    let mut app = App::new();
    app
        .add_state::<AppState>()
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugin(EditorPlugin::default())
        .add_plugin(CameraPlugin)
        .add_plugin(NetworkPlugin)
        .add_plugin(MenuPlugin)
        .add_plugin(LevelPlugin)
        .add_plugin(GamepadPlugin);
    #[cfg(feature = "debug")]
    {
        app.add_plugin(RapierDebugRenderPlugin::default());
    }
    app.run();
}

// Time Energy
pub struct Ether(f64);
