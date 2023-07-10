use bevy::prelude::*;
use bevy::asset::Assets;
use bevy::pbr::{PbrBundle, PointLight, PointLightBundle, StandardMaterial};
use crate::app_state::AppState;
use crate::player::{Player, Velocity};
use bevy_ggrs::ggrs::{ PlayerHandle };
use bevy_ggrs::{RollbackIdProvider, PlayerInputs, Rollback, Session, GGRSSchedule};
use nalgebra::ComplexField;
use crate::input::{INPUT_MOVE_DOWN, INPUT_MOVE_UP};
use crate::network::GgrsConfig;

pub struct LevelPlugin;

const MOVEMENT_SPEED: f32 = 0.005;
const MAX_SPEED: f32 = 0.05;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(setup_scene.in_schedule(OnEnter(AppState::Online)))
            .add_system(spawn_players.in_schedule(OnEnter(AppState::Online)))
            .add_system(move_player_system.in_schedule(GGRSSchedule));
    }
}

fn move_player_system(
    mut query: Query<(&mut Velocity, &mut Transform, &Player), With<Rollback>>,
    inputs: Res<PlayerInputs<GgrsConfig>>,
) {
    for (mut vel, mut transform, player) in query.iter_mut() {
        let input = inputs[player.handle].0;
        let mut v = vel.0;
        v.x += input.axis_lx * MOVEMENT_SPEED;
        v.z += input.axis_ly * MOVEMENT_SPEED;
        let mag = ComplexField::sqrt(v.x * v.x + v.y * v.y + v.z * v.z);
        if mag > MAX_SPEED {
            let factor = MAX_SPEED / mag;
            v.x *= factor;
            v.y *= factor;
            v.z *= factor;
        }
        transform.translation += v;
    }
}

fn spawn_players(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut rip: ResMut<RollbackIdProvider>,
    session: Res<Session<GgrsConfig>>,
) {
    let num_players = match &*session {
        Session::SyncTestSession(s) => s.num_players(),
        Session::P2PSession(s) => s.num_players(),
        Session::SpectatorSession(s) => s.num_players(),
    };
    for handle in 0..num_players {
        commands.spawn((
            PbrBundle {
                mesh: meshes.add(shape::UVSphere::default().into()),
                material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
                transform: Transform::from_xyz(0.0, 0.5, 0.0),
                ..default()
            },
            Player { handle },
            Velocity::default(),
            rip.next(),
        ));
    }
}

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Plane::from_size(5.0).into()),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });
    // cube
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    });
    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
}
