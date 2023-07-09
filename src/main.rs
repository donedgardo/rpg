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
use bevy::prelude::*;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .run();
}

fn setup(mut commands: Commands, mut materials: ResMut<StandardMaterial>) {
    // Add camera
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(3.0, 5.0, -8.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });

    // Add light source
    commands.spawn_bundle(LightBundle {
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });

    // Add a cube
    commands.spawn_bundle(PbrBundle {
        mesh: Mesh::from(shape::Cube { size: 2.0 }),
        material: materials.add(Color::rgb(0.2, 0.2, 0.8).into()),
        ..Default::default()
    });
}
use bevy::prelude::*;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .run();
}

fn setup(mut commands: Commands, mut materials: ResMut<StandardMaterial>) {
    // Add camera
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(3.0, 5.0, -8.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });

    // Add light source
    commands.spawn_bundle(LightBundle {
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });

    // Add a cube
    commands.spawn_bundle(PbrBundle {
        mesh: Mesh::from(shape::Cube { size: 2.0 }),
        material: materials.add(Color::rgb(0.2, 0.2, 0.8).into()),
        ..Default::default()
    });
}
use bevy::prelude::*;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .run();
}

fn setup(mut commands: Commands, mut materials: ResMut<StandardMaterial>) {
    // Add camera
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(3.0, 5.0, -8.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });

    // Add light source
    commands.spawn_bundle(LightBundle {
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });

    // Add a cube
    commands.spawn_bundle(PbrBundle {
        mesh: Mesh::from(shape::Cube { size: 2.0 }),
        material: materials.add(Color::rgb(0.2, 0.2, 0.8).into()),
        ..Default::default()
    });
}
use bevy::prelude::*;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .run();
}

fn setup(mut commands: Commands, mut materials: ResMut<StandardMaterial>) {
    // Add camera
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(3.0, 5.0, -8.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });

    // Add light source
    commands.spawn_bundle(LightBundle {
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });

    // Add a cube
    commands.spawn_bundle(PbrBundle {
        mesh: Mesh::from(shape::Cube { size: 2.0 }),
        material: materials.add(Color::rgb(0.2, 0.2, 0.8).into()),
        ..Default::default()
    });
}
