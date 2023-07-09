use bevy::prelude::*;
use bevy::input::gamepad::{GamepadConnection, GamepadConnectionEvent, GamepadEvent};

pub struct GamepadPlugin;

impl Plugin for GamepadPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(gamepad_connections);
    }
}

#[derive(Resource)]
pub struct MyGamepad(pub Gamepad);

fn gamepad_connections(
    mut commands: Commands,
    my_gamepad: Option<Res<MyGamepad>>,
    mut gamepad_evr: EventReader<GamepadConnectionEvent>,
) {
    // the ID of the gamepad
    for info in gamepad_evr.iter() {
        match info.connection {
            GamepadConnection::Connected(_) => {
                dbg!("New gamepad connected with ID: {:?}", info.gamepad.id);
                if my_gamepad.is_none() {
                    commands.insert_resource(MyGamepad(info.gamepad));
                }
            }
            GamepadConnection::Disconnected => {
                dbg!("Lost gamepad connection with ID: {:?}", info.gamepad.id);
                if let Some(MyGamepad(old_id)) = my_gamepad.as_deref() {
                    if *old_id == info.gamepad {
                        commands.remove_resource::<MyGamepad>();
                    }
                }
            }
        }
    }
}