use bevy::input::gamepad::GamepadEvent;
use bevy::prelude::*;
use bevy_ggrs::ggrs::PlayerHandle;
use crate::gamepad::MyGamepad;

pub const INPUT_JUMP: u8 = 1 << 0;
pub const INPUT_ATTACK: u8 = 1 << 1;
pub const INPUT_BLOCK: u8 = 1 << 2;
pub const INPUT_SPECIAL: u8 = 1 << 3;
pub const INPUT_MOVE_UP: u8 = 1 << 4;
pub const INPUT_MOVE_DOWN: u8 = 1 << 5;
pub const INPUT_MOVE_LEFT: u8 = 1 << 6;
pub const INPUT_MOVE_RIGHT: u8 = 1 << 7;

pub fn input(
    _: In<PlayerHandle>,
    buttons: Res<Input<GamepadButton>>,
    axes: Res<Axis<GamepadAxis>>,
    my_gamepad: Option<Res<MyGamepad>>,
) -> u8 {
    let mut input = 0u8;
    let gamepad = if let Some(gp) = my_gamepad {
        // a gamepad is connected, we have the id
        gp.0
    } else {
        // no gamepad is connected
        return input;
    };

    // The joysticks are represented using a separate axis for X and Y
    let axis_lx = GamepadAxis {
        gamepad, axis_type: GamepadAxisType::LeftStickX
    };
    let axis_ly = GamepadAxis {
        gamepad, axis_type: GamepadAxisType::LeftStickY
    };

    if let (Some(x), Some(y)) = (axes.get(axis_lx), axes.get(axis_ly)) {
        // combine X and Y into one vector
        let left_stick_pos = Vec2::new(x, y);
        // Example: check if the stick is pushed up
        if left_stick_pos.length() > 0.9 {
            if left_stick_pos.y > 0.5 {
                input |= INPUT_MOVE_UP
            } else {
                input |= INPUT_MOVE_DOWN
            }
        }
    }

    // In a real game, the buttons would be configurable, but here we hardcode them
    let jump_button = GamepadButton {
        gamepad, button_type: GamepadButtonType::South
    };
    let attack_button = GamepadButton {
        gamepad, button_type: GamepadButtonType::East
    };
    let parry_button =  GamepadButton {
        gamepad, button_type: GamepadButtonType::West
    };

    if buttons.just_pressed(jump_button) {
        input |= INPUT_JUMP;
    }
    if buttons.just_pressed(attack_button) {
        input |= INPUT_ATTACK;
    }
    if buttons.just_pressed(parry_button) {
        input |= INPUT_BLOCK;
    }
    input
}
