use bevy::prelude::*;
use bevy_ggrs::ggrs::{Config, SessionBuilder};
use bevy_ggrs::GGRSPlugin;
use bevy_matchbox::prelude::*;
use crate::input::input;

const FPS: usize = 60;

pub struct NetworkPlugin;
impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut App) {
        integrate_ggrs_plugin(app);
        app
            .add_startup_system(start_socket)
            .add_system(wait_for_players)
            .insert_resource(PlayerConfig { num_players: 2});
    }
}

pub struct GgrsConfig;
impl Config for GgrsConfig {
    // 4-directions + fire fits easily in a single byte
    type Input = u8;
    type State = u8;
    // Matchbox' WebRtcSocket addresses are called `PeerId`s
    type Address = PeerId;
}

#[derive(Resource)]
pub struct PlayerConfig {
    pub num_players: usize,
}

pub fn start_socket(mut commands: Commands, player_config: Res<PlayerConfig>) {
    let room_url = format!("ws://127.0.0.1:3536/arena?next={}", player_config.num_players);
    info!("connecting to server: {:?}", room_url);
    commands.insert_resource(MatchboxSocket::new_ggrs(&room_url));
}

pub fn wait_for_players(
    mut commands: Commands,
    mut socket: ResMut<MatchboxSocket<SingleChannel>>,
    player_config: Res<PlayerConfig>
) {
    if socket.get_channel(0).is_err() {
        return;
    }
    socket.update_peers();
    let players = socket.players();

    if players.len() < player_config.num_players {
        return; // wait for more players
    }

    info!("All peers have joined, going in-game");

    let mut session_builder = SessionBuilder::<GgrsConfig>::new()
        .with_num_players(player_config.num_players)
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


pub fn integrate_ggrs_plugin(app: &mut App) {
    GGRSPlugin::<GgrsConfig>::new()
        // define frequency of rollback game logic update
        .with_update_frequency(FPS)
        // define system that returns inputs given a player handle, so GGRS can send the inputs around
        .with_input_system(input)
        .build(app);
}

