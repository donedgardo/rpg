use std::collections::{HashMap};
use std::collections::vec_deque::VecDeque;
use std::time::{Duration, Instant};
use bevy::prelude::*;
use bevy::asset::Assets;
use bevy::ecs::schedule::{LogLevel, ScheduleBuildSettings, ScheduleLabel};
use bevy::sprite::{MaterialMesh2dBundle};
use bevy::render::color::Color;
use crate::app_state::AppState;
use crate::player::{Player, Velocity};
use crate::input::MyGameInput;
use bevy_ggrs::{RollbackIdProvider, Session, GGRSSchedule};
use bevy_ggrs::ggrs::PlayerHandle;
use crate::player_movement::{move_local_player_system, move_online_player_system};
use crate::network::GgrsConfig;

const MAX_FRAMES: usize = 15;

#[derive(Resource, Default)]
struct InputSnapshots {
    snapshots: HashMap<PlayerHandle, VecDeque<MyGameInput>>,
}

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
            .add_system(spawn_players.in_schedule(OnEnter(AppState::Online)))
            .add_system(move_online_player_system.in_schedule(GGRSSchedule));

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
            .add_system(move_local_player_system.in_schedule(LocalPlaySchedule))
            .add_system(input_snapshot_system.in_schedule(LocalPlaySchedule));
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

fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    handle: usize,
    position: f32,
) {
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes
                .add(shape::Quad::new(Vec2::new(50., 50.)).into())
                .into(),
            material: materials.add(ColorMaterial::from(Color::RED)),
            transform: Transform::from_translation(Vec3::new(position, 50., 0.)),
            ..default()
        },
        Player { handle },
        Velocity::default(),
    ));
}

fn spawn_local_players(
    commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>,
) {
    // Spawn a single player for local play
    spawn_player(commands, meshes, materials, 0, -50.);
}

fn spawn_players(
    commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>,
    mut rip: ResMut<RollbackIdProvider>,
    session: Res<Session<GgrsConfig>>,
) {
    let num_players = match &*session {
        Session::SyncTestSession(s) => s.num_players(),
        Session::P2PSession(s) => s.num_players(),
        Session::SpectatorSession(s) => s.num_players(),
    };
    for handle in 0..num_players {
        let position = if handle == 0 { -50. } else { 50. };
        spawn_player(commands, meshes, materials, handle, position);
        commands.with(rip.next());
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
fn input_snapshot_system(
    mut input_snapshots: ResMut<InputSnapshots>,
    query: Query<(&Player, &MyGameInput)>,
) {
    for (player, input) in query.iter() {
        let player_snapshots = input_snapshots
            .snapshots
            .entry(player.handle)
            .or_insert_with(VecDeque::new);
        if player_snapshots.len() >= MAX_FRAMES {
            player_snapshots.pop_front();
        }
        player_snapshots.push_back(*input);
    }
}
