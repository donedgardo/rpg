use bevy::prelude::*;
use bevy_ggrs::ggrs::PlayerHandle;

pub const INPUT_JUMP: u8 = 1 << 0;
pub const INPUT_ATTACK: u8 = 1 << 1;
pub const INPUT_BLOCK: u8 = 1 << 2;
pub const INPUT_SPECIAL: u8 = 1 << 3;
pub const INPUT_MOVE_X: u8 = 1 << 4;
pub const INPUT_MOVE_Y: u8 = 1 << 5;
pub const INPUT_MOVE_Z: u8 = 1 << 6;
pub fn input(_: In<PlayerHandle>, gamepad: Res<Input<GamepadButton>>) -> u8 {
    let mut input = 0u8;
    if gamepad.just_pressed(GamepadButtonType::South) {
        input |= INPUT_JUMP;
    }
    if gamepad.just_pressed(GamepadButton::East) {
        input |= INPUT_ATTACK;
    }
    if gamepad.just_pressed(GamepadButton::West) {
        input |= INPUT_BLOCK;
    }
    if gamepad.just_pressed(GamepadButton::North) {
        input |= INPUT_SPECIAL;
    }
    if gamepad.axis_value(GamepadAxis::LeftStickX).unwrap_or(0.0) > 0.0 {
        input |= INPUT_MOVE_X;
    }
    if gamepad.axis_value(GamepadAxis::LeftStickY).unwrap_or(0.0) > 0.0 {
        input |= INPUT_MOVE_Y;
    }
    if gamepad.axis_value(GamepadAxis::RightStickY).unwrap_or(0.0) > 0.0 {
        input |= INPUT_MOVE_Z;
    }
    input
}
