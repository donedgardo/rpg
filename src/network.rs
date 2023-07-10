use bevy::prelude::*;
use bevy_ggrs::ggrs::{Config, SessionBuilder};
use bevy_ggrs::GGRSPlugin;
use bevy_matchbox::prelude::*;
use crate::app_state::AppState;
use crate::input::{input, MyGameInput};
use crate::player::Velocity;

const FPS: usize = 60;
const WEBSOCKET_SIGNAL_SERVER: &'static str = "127.0.0.1:3536";

pub struct NetworkPlugin;

impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut App) {
        integrate_ggrs_plugin(app);
        app
            .add_system(start_socket.in_schedule(OnEnter(AppState::WaitingForPlayers)))
            .add_system(wait_for_players.in_set(OnUpdate(AppState::WaitingForPlayers)))
            .insert_resource(PlayerConfig { num_players: 2 });
    }
}

pub struct GgrsConfig;

impl Config for GgrsConfig {
    type Input = MyGameInput;
    type State = u8;
    // Matchbox' WebRtcSocket addresses are called `PeerId`s
    type Address = PeerId;
}

#[derive(Resource)]
pub struct PlayerConfig {
    pub num_players: usize,
}


pub fn start_socket(mut commands: Commands, player_config: Res<PlayerConfig>) {
    let room_url = format!("ws://{}/arena?next={}", WEBSOCKET_SIGNAL_SERVER, player_config.num_players);
    info!("connecting to server: {:?}", room_url);
    commands.insert_resource(MatchboxSocket::new_ggrs(&room_url));
}

pub fn wait_for_players(
    mut commands: Commands,
    mut socket: ResMut<MatchboxSocket<SingleChannel>>,
    player_config: Res<PlayerConfig>,
    mut app_state: ResMut<NextState<AppState>>,
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

    // P2P Session
    // start the GGRS session
    //let ggrs_session = session_builder
    //    .start_p2p_session(channel)
    //    .expect("failed to start session");
    //commands.insert_resource(bevy_ggrs::Session::P2PSession(ggrs_session));

    // TEST
    let test_session = session_builder.start_synctest_session()
        .expect("failed to start test session");
    commands.insert_resource(bevy_ggrs::Session::SyncTestSession(test_session));
    app_state.set(AppState::Online);
}


pub fn integrate_ggrs_plugin(app: &mut App) {
    GGRSPlugin::<GgrsConfig>::new()
        // define frequency of rollback game logic update
        .with_update_frequency(FPS)
        // define system that returns inputs given a player handle, so GGRS can send the inputs around
        .with_input_system(input)
        .register_rollback_component::<Transform>()
        .register_rollback_component::<Velocity>()
        .build(app);
}

