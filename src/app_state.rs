use bevy::prelude::States;

#[derive(States, Clone, Eq, PartialEq, Debug, Hash, Default)]
pub enum AppState {
    #[default]
    Menu,
    Online,
}
