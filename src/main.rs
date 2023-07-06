use bevy::prelude::*;
use bevy_ggrs::ggrs::{Config, SessionBuilder, PlayerHandle};
use bevy_ggrs::GGRSPlugin;
use bevy_matchbox::prelude::*;

mod arena;
mod player;
mod camera;

use camera::CameraPlugin;

struct GgrsConfig;

const FPS: usize = 60;

const INPUT_UP: u8 = 1 << 0;
const INPUT_DOWN: u8 = 1 << 1;
const INPUT_LEFT: u8 = 1 << 2;
const INPUT_RIGHT: u8 = 1 << 3;
const INPUT_FIRE: u8 = 1 << 4;

fn input(_: In<PlayerHandle>, keys: Res<Input<KeyCode>>) -> u8 {
    let mut input = 0u8;
    if keys.any_pressed([KeyCode::Up, KeyCode::W]) {
        input |= INPUT_UP;
    }
    if keys.any_pressed([KeyCode::Down, KeyCode::S]) {
        input |= INPUT_DOWN;
    }
    if keys.any_pressed([KeyCode::Left, KeyCode::A]) {
        input |= INPUT_LEFT
    }
    if keys.any_pressed([KeyCode::Right, KeyCode::D]) {
        input |= INPUT_RIGHT;
    }
    if keys.any_pressed([KeyCode::Space, KeyCode::Return]) {
        input |= INPUT_FIRE;
    }
    input
}

impl Config for GgrsConfig {
    // 4-directions + fire fits easily in a single byte
    type Input = u8;
    type State = u8;
    // Matchbox' WebRtcSocket addresses are called `PeerId`s
    type Address = PeerId;
}

fn start_socket(mut commands: Commands) {
    let room_url = "ws://127.0.0.1:3536/arena?next=2";
    info!("connecting to server: {:?}", room_url);
    commands.insert_resource(MatchboxSocket::new_ggrs(room_url));
}

fn wait_for_players(
    mut commands: Commands,
    mut socket: ResMut<MatchboxSocket<SingleChannel>>
) {
    if socket.get_channel(0).is_err() {
        return;
    }
    socket.update_peers();
    let players = socket.players();

    let num_players = 2;
    if players.len() < num_players {
        return; // wait for more players
    }

    info!("All peers have joined, going in-game");

    let mut session_builder = SessionBuilder::<GgrsConfig>::new()
        .with_num_players(num_players)
        .with_input_delay(2);

    for (i, player) in players.into_iter().enumerate() {
        session_builder = session_builder
            .add_player(player, i)
            .expect("failed to add player");
    }

    // move the channel out of the socket (required because GGRS takes ownership of it)
    let channel = socket.take_channel(0).unwrap();

    // start the GGRS session
    let ggrs_session = session_builder
        .start_p2p_session(channel)
        .expect("failed to start session");

    commands.insert_resource(bevy_ggrs::Session::P2PSession(ggrs_session));
}

fn main() {

    let mut app = App::new();
    GGRSPlugin::<GgrsConfig>::new()
        // define frequency of rollback game logic update
        .with_update_frequency(FPS)
        // define system that returns inputs given a player handle, so GGRS can send the inputs around
        .with_input_system(input)
        .build(&mut app);
    app
        .add_plugins(DefaultPlugins)
        .add_startup_system(start_socket)
        .add_system(wait_for_players)
        .run();
}

// Time Energy
pub struct Ether(f64);

// Magnetic field
