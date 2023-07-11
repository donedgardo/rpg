use bevy::prelude::*;
use bevy::input::gamepad::{GamepadConnection, GamepadConnectionEvent, GamepadEvent};
use nalgebra::ComplexField;

const DEADZONE: f32 = 0.2;

const DEADZONE: f32 = 0.2;

pub struct GamepadPlugin;

pub struct GamepadAxes {
    pub lx: f32,
    pub ly: f32,
}

impl GamepadAxes {
    pub fn apply_deadzone(&mut self) {
        if ComplexField::abs(self.lx) < DEADZONE {
            self.lx = 0.0;
        }
        if ComplexField::abs(self.ly) < DEADZONE {
            self.ly = 0.0;
        }
    }
}

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