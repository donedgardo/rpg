use bevy::prelude::*;
use app_state::AppState;
use camera::CameraPlugin;
use network::NetworkPlugin;
use menu::MenuPlugin;
use bevy_rapier2d::prelude::*;
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
mod player_movement;

fn main() {
    let mut app = App::new();
    app
        .add_state::<AppState>()
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<()>::pixels_per_meter(50.))
        .add_plugin(CameraPlugin)
        .add_plugin(NetworkPlugin)
        .add_plugin(MenuPlugin)
        .add_plugin(LevelPlugin)
        .add_plugin(GamepadPlugin);
    #[cfg(feature = "debug")]
    {
        use bevy_editor_pls::prelude::*;
        app
            .add_plugin(RapierDebugRenderPlugin::default())
            .add_plugin(EditorPlugin::default());
    }
    app.run();
}

// Time Energy
pub struct Ether(f64);
