use bevy::prelude::*;
use bevy::asset::Assets;
use bevy::sprite::{MaterialMesh2dBundle, SpriteBundle};
use bevy::render::color::Color;
use crate::app_state::AppState;
use crate::player::{Player, Velocity};
use bevy_ggrs::ggrs::{PlayerHandle};
use bevy_ggrs::{RollbackIdProvider, PlayerInputs, Rollback, Session, GGRSSchedule};
use nalgebra::ComplexField;
use crate::input::{INPUT_MOVE_DOWN, INPUT_MOVE_UP};
use crate::network::GgrsConfig;

pub struct LevelPlugin;

const MOVEMENT_SPEED: f32 = 1.;
const MAX_SPEED: f32 = 5.;

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
        v.y += input.axis_lx * MOVEMENT_SPEED;
        v.x += input.axis_ly * MOVEMENT_SPEED;
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
    mut materials: ResMut<Assets<ColorMaterial>>,
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
            MaterialMesh2dBundle {
                mesh: meshes
                    .add(shape::Quad::new(Vec2::new(50., 50.)).into())
                    .into(),
                material: materials.add(ColorMaterial::from(Color::RED)),
                transform: Transform::from_translation(
                    Vec3::new(if handle == 0 { -50. } else { 50. }, 50., 0.)
                ),
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
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Quad
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes
            .add(shape::Quad::new(Vec2::new(600., 200.)).into())
            .into(),
        material: materials.add(ColorMaterial::from(Color::GREEN)),
        transform: Transform::from_translation(Vec3::new(0., -100., 0.)),
        ..default()
    });
}
