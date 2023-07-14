use bevy::prelude::*;
use bevy_ggrs::{PlayerInputs, Rollback};
use nalgebra::ComplexField;
use crate::player::{Player, Velocity};
use crate::gamepad::{GamepadAxes, MyGamepad};
use crate::network::GgrsConfig;

const MOVEMENT_SPEED: f32 = 1.;
const MAX_SPEED: f32 = 5.;

pub fn move_player(
    vel: &mut Velocity,
    transform: &mut Transform,
    x: f32,
) {
    let mut v = vel.0;
    v.x += x * MOVEMENT_SPEED;
    let mag = ComplexField::sqrt(v.x * v.x + v.y * v.y + v.z * v.z);
    if mag > MAX_SPEED {
        let factor = MAX_SPEED / mag;
        v.x *= factor;
        v.y *= factor;
        v.z *= factor;
    }
    transform.translation += v;
}

pub fn move_local_player_system(
    mut query: Query<(&mut Velocity, &mut Transform, &Player)>,
    my_gamepad: Option<Res<MyGamepad>>,
    axes: Res<Axis<GamepadAxis>>,
) {
    let gamepad = if let Some(gp) = my_gamepad {
        gp.0
    } else {
        return;
    };
    let axis_lx = GamepadAxis {
        gamepad,
        axis_type: GamepadAxisType::LeftStickX,
    };
    for (mut vel, mut transform, _) in query.iter_mut() {
        if let Some(x) = axes.get(axis_lx) {
            let mut gamepad_axes = GamepadAxes {
                lx: x,
                ly: 0.,
            };
            gamepad_axes.apply_deadzone();
            move_player(&mut vel, &mut transform, gamepad_axes.lx);
        }
    }
}

pub fn move_online_player_system(
    mut query: Query<(&mut Velocity, &mut Transform, &Player), With<Rollback>>,
    inputs: Res<PlayerInputs<GgrsConfig>>,
) {
    for (mut vel, mut transform, player) in query.iter_mut() {
        let input = inputs[player.handle].0;
        move_player(&mut vel, &mut transform, input.axis_lx);
    }
}
