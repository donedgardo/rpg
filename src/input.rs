use std::collections::HashMap;
use bevy::prelude::*;
use bevy_ggrs::ggrs::PlayerHandle;
use bytemuck::{Pod, Zeroable};
use nalgebra::ComplexField;
use std::collections::vec_deque::VecDeque;
use crate::gamepad::{GamepadAxes, MyGamepad};

use crate::player::Player;

pub const MAX_FRAMES: usize = 15;

pub const INPUT_JUMP: u8 = 1 << 0;
pub const INPUT_ATTACK: u8 = 1 << 1;
pub const INPUT_BLOCK: u8 = 1 << 2;
pub const INPUT_SPECIAL: u8 = 1 << 3;
pub const INPUT_MOVE_UP: u8 = 1 << 4;
pub const INPUT_MOVE_DOWN: u8 = 1 << 5;
pub const INPUT_MOVE_LEFT: u8 = 1 << 6;
pub const INPUT_MOVE_RIGHT: u8 = 1 << 7;

#[repr(C, packed)]
#[derive(Copy, Clone, Pod, Zeroable, Default)]
pub struct MyGameInput {
    pub axis_lx: f32,
    pub axis_ly: f32,
    pub actions: u8,
}

#[derive(Resource, Default)]
pub struct InputSnapshots {
    pub snapshots: HashMap<PlayerHandle, VecDeque<MyGameInput>>,
}

impl PartialEq for MyGameInput {
    fn eq(&self, other: &Self) -> bool {
        are_approx_equal(self.axis_lx, other.axis_lx) &&
            are_approx_equal(self.axis_ly, other.axis_ly) &&
            self.actions == other.actions
    }
}

fn are_approx_equal(a: f32, b: f32) -> bool {
    let epsilon = 0.00001;
    ComplexField::abs(a - b) < epsilon
}

pub fn ggrs_input_system(
    _: In<PlayerHandle>,
    buttons: Res<Input<GamepadButton>>,
    axes: Res<Axis<GamepadAxis>>,
    my_gamepad: Option<Res<MyGamepad>>,
) -> MyGameInput {
    get_my_game_input(&buttons, &axes, &my_gamepad)
}

pub fn get_my_game_input(buttons: &Res<Input<GamepadButton>>, axes: &Res<Axis<GamepadAxis>>, my_gamepad: &Option<Res<MyGamepad>>) -> MyGameInput {
    let mut input = MyGameInput::default();
    let gamepad = if let Some(gp) = my_gamepad {
        gp.0
    } else {
        return input;
    };

    let axis_lx = GamepadAxis {
        gamepad,
        axis_type: GamepadAxisType::LeftStickX,
    };
    let axis_ly = GamepadAxis {
        gamepad,
        axis_type: GamepadAxisType::LeftStickY,
    };

    if let (Some(x), Some(y)) = (axes.get(axis_lx), axes.get(axis_ly)) {
        let mut gamepad_axes = GamepadAxes {
            lx: x,
            ly: y,
        };
        gamepad_axes.apply_deadzone();
        input.axis_ly = gamepad_axes.ly;
        input.axis_lx = gamepad_axes.lx;
    }

    let jump_button = GamepadButton {
        gamepad,
        button_type: GamepadButtonType::South,
    };
    let attack_button = GamepadButton {
        gamepad,
        button_type: GamepadButtonType::East,
    };
    let parry_button = GamepadButton {
        gamepad,
        button_type: GamepadButtonType::West,
    };

    if buttons.just_pressed(jump_button) {
        input.actions |= INPUT_JUMP;
    }
    if buttons.just_pressed(attack_button) {
        input.actions |= INPUT_ATTACK;
    }
    if buttons.just_pressed(parry_button) {
        input.actions |= INPUT_BLOCK;
    }
    input
}

pub fn input_snapshot_system(
    mut input_snapshots: ResMut<InputSnapshots>,
    buttons: Res<Input<GamepadButton>>,
    axes: Res<Axis<GamepadAxis>>,
    my_gamepad: Option<Res<MyGamepad>>,
    query: Query<(&Player)>,
) {
    // Todo get multiple controller inputs bases on player
    for (player) in query.iter() {
        let first_player_input = get_my_game_input(&buttons, &axes, &my_gamepad);
        let player_snapshots = input_snapshots
            .snapshots
            .entry(player.handle)
            .or_insert_with(VecDeque::new);
        if player_snapshots.len() >= MAX_FRAMES {
            player_snapshots.pop_front();
        }
        player_snapshots.push_back(first_player_input);
    }
}
