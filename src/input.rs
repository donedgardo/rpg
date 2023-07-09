use bevy::prelude::*;
use bevy_ggrs::ggrs::PlayerHandle;

pub const INPUT_JUMP: u8 = 1 << 0;
pub const INPUT_ATTACK: u8 = 1 << 1;
pub const INPUT_BLOCK: u8 = 1 << 2;
pub const INPUT_SPECIAL: u8 = 1 << 3;
pub const INPUT_MOVE_X: u8 = 1 << 4;
pub const INPUT_MOVE_Y: u8 = 1 << 5;
pub const INPUT_MOVE_Z: u8 = 1 << 6;
pub fn input(_: In<PlayerHandle>, gamepad: Res<Input<Gamepad>>) -> u8 {
    let mut input = 0u8;
    if gamepad.button_pressed(GamepadButton::South(0)) {
        input |= INPUT_JUMP;
    }
    if gamepad.button_pressed(GamepadButton::East(0)) {
        input |= INPUT_ATTACK;
    }
    if gamepad.button_pressed(GamepadButton::West(0)) {
        input |= INPUT_BLOCK;
    }
    if gamepad.button_pressed(GamepadButton::North(0)) {
        input |= INPUT_SPECIAL;
    }
    if gamepad.axis_value(GamepadAxis::LeftStickX(0)).unwrap_or(0.0) > 0.0 {
        input |= INPUT_MOVE_X;
    }
    if gamepad.axis_value(GamepadAxis::LeftStickY(0)).unwrap_or(0.0) > 0.0 {
        input |= INPUT_MOVE_Y;
    }
    if gamepad.axis_value(GamepadAxis::RightStickY(0)).unwrap_or(0.0) > 0.0 {
        input |= INPUT_MOVE_Z;
    }
    input
}
