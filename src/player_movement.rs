use std::collections::vec_deque::VecDeque;
use bevy::prelude::*;
use bevy_ggrs::{PlayerInputs, Rollback};
use bevy_rapier2d::control::KinematicCharacterController;
use nalgebra::ComplexField;
use crate::player::{Player, Velocity};
use crate::gamepad::{GamepadAxes, MyGamepad};
use crate::input::{InputSnapshots, GGRSGameInput};
use crate::network::GgrsConfig;

const MOVEMENT_SPEED: f32 = 1.;
const MAX_SPEED: f32 = 5.;

pub fn move_player(
    controller: &mut KinematicCharacterController,
    lx: f32,
) {
    let mut v = Vec2::ZERO;
    v.x += lx * MOVEMENT_SPEED;
    let mag = ComplexField::sqrt(v.x * v.x + v.y * v.y);
    if mag > MAX_SPEED {
        let factor = MAX_SPEED / mag;
        v.x *= factor;
        v.y *= factor;
    }
    controller.translation = Some(v);
}

pub fn move_local_player_system(
    mut query: Query<(&mut KinematicCharacterController, &Player)>,
    input_snapshots: Res<InputSnapshots>,
) {
    for (mut controller, player) in query.iter_mut() {
        match input_snapshots.snapshots.get(&player.handle) {
            None => {
                println!("No snapshot found for player {}", player.handle);
            }
            Some(inputs) => {
                if let Some(input) = inputs.back() {
                    let mut gamepad_axes = GamepadAxes {
                        lx: input.axis_lx,
                        ly: 0.,
                    };
                    gamepad_axes.apply_deadzone();
                    move_player(&mut controller, gamepad_axes.lx);
                }
            }
        }

    }
}

pub fn move_online_player_system(
    mut query: Query<(&mut KinematicCharacterController, &Player), With<Rollback>>,
    inputs: Res<PlayerInputs<GgrsConfig>>,
) {
    for (mut controller, player) in query.iter_mut() {
        let input = inputs[player.handle].0;
        move_player(&mut controller, input.axis_lx);
    }
}
