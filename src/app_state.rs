use bevy::prelude::States;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum AppState {
    Menu,
    Online,
    WaitingForPlayers,
}
}
