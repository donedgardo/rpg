use std::collections::HashMap;
use std::collections::vec_deque::VecDeque;
use std::time::{Duration, Instant};
use bevy::prelude::*;
use bevy::asset::Assets;
use bevy::ecs::schedule::{LogLevel, ScheduleBuildSettings, ScheduleLabel};
use bevy::ecs::system::EntityCommands;
use bevy::sprite::MaterialMesh2dBundle;
use bevy::render::color::Color;
use crate::app_state::AppState;
use crate::player::{Player, Velocity};
use crate::input::{get_my_game_input, InputSnapshots, GGRSGameInput};
use bevy_ggrs::{GGRSSchedule, Rollback, RollbackIdProvider, Session};
use bevy_ggrs::ggrs::PlayerHandle;
use bevy_rapier2d::control::KinematicCharacterController;
use bevy_rapier2d::geometry::Collider;
use bevy_rapier2d::prelude::RigidBody;
use crate::gamepad::MyGamepad;
use crate::input;
use crate::player_movement::{move_local_player_system, move_online_player_system};
use crate::network::GgrsConfig;


pub struct LevelPlugin;

#[derive(ScheduleLabel, Debug, Hash, PartialEq, Eq, Clone)]
pub struct LocalPlaySchedule;

#[derive(Resource)]
struct LocalPlayStage {
    update_frequency: usize,
    last_updated: Instant,
    accumulator: Duration,
}

impl LocalPlayStage {
    pub fn new() -> Self {
        Self {
            update_frequency: 60,
            last_updated: Instant::now(),
            accumulator: Duration::ZERO,
        }
    }
}

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        // Online
        app.add_system(setup_scene.in_schedule(OnEnter(AppState::Online)))
            .add_system(spawn_network_players.in_schedule(OnEnter(AppState::Online)))
            .add_systems((input::input_snapshot_system, move_online_player_system).chain().in_schedule(GGRSSchedule));

        //Local
        let mut schedule = Schedule::default();
        schedule.set_build_settings(ScheduleBuildSettings {
            ambiguity_detection: LogLevel::Error,
            ..default()
        });
        let stage = LocalPlayStage::new();
        app.add_schedule(LocalPlaySchedule, schedule);
        app.add_system(local_play_schedule_system.in_base_set(CoreSet::PreUpdate));
        app.insert_resource(stage);
        app.add_system(setup_scene.in_schedule(OnEnter(AppState::LocalPlay)))
            .add_system(spawn_local_players.in_schedule(OnEnter(AppState::LocalPlay)))
            .add_systems((input::input_snapshot_system, move_local_player_system).chain().in_schedule(LocalPlaySchedule));
        app.insert_resource(InputSnapshots::default());
    }
}

fn local_play_schedule_system(world: &mut World) {
    let state = world.get_resource::<State<AppState>>()
        .expect("failed to extract game state");
    if state.0 != AppState::LocalPlay { return; };
    let mut stage = world
        .remove_resource::<LocalPlayStage>()
        .expect("failed to extract local play schedule");
    let delta = Instant::now().duration_since(stage.last_updated);
    let fps_delta = 1. / stage.update_frequency as f64;
    stage.accumulator = stage.accumulator.saturating_add(delta);
    stage.last_updated = Instant::now();
    while stage.accumulator.as_secs_f64() > fps_delta {
        // decrease accumulator
        stage.accumulator = stage
            .accumulator
            .saturating_sub(Duration::from_secs_f64(fps_delta));
        world.run_schedule(LocalPlaySchedule);
    }
    world.insert_resource(stage);
}

fn spawn_local_players(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Spawn a single player for local play
    commands.spawn(
        player_bundle_common(&mut meshes, &mut materials, 0, Vec3::new(-50., 50., 0.))
    );
}

fn spawn_network_players(
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
        let position = if handle == 0 { Vec3::new(-50., 50., 0.) } else { Vec3::new(50., 50., 0.) };
        commands.spawn(player_bundle_common(&mut meshes, &mut materials, handle, position))
            .insert(rip.next());
    }
}

#[derive(Bundle)]
struct PlayerBundle {
    mesh_bundle: MaterialMesh2dBundle<ColorMaterial>,
    player: Player,
    rigid_body: RigidBody,
    collider: Collider,
    controller: KinematicCharacterController,
}


fn player_bundle_common(
    mut meshes: &mut ResMut<Assets<Mesh>>,
    mut materials: &mut ResMut<Assets<ColorMaterial>>,
    handle: usize,
    position: Vec3,
) -> PlayerBundle {
    PlayerBundle {
        mesh_bundle: MaterialMesh2dBundle {
            mesh: meshes
                .add(shape::Quad::new(Vec2::new(50., 50.)).into())
                .into(),
            material: materials.add(ColorMaterial::from(Color::RED)),
            transform: Transform::from_translation(position),
            ..default()
        },
        player: Player { handle },
        rigid_body: RigidBody::KinematicPositionBased,
        collider: Collider::capsule(Vec2::ZERO, Vec2::new(0., 50.), 25.),
        controller: KinematicCharacterController::default(),
    }
}

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Quad
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes
                .add(shape::Quad::new(Vec2::new(600., 200.)).into())
                .into(),
            material: materials.add(ColorMaterial::from(Color::GREEN)),
            transform: Transform::from_translation(Vec3::new(0., -100., 0.)),
            ..default()
        })
    );
}
