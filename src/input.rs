use bevy::prelude::*;
use bevy_ggrs::ggrs::PlayerHandle;

pub const INPUT_JUMP: u8 = 1 << 0;
pub const INPUT_ATTACK: u8 = 1 << 1;
pub const INPUT_JUMP: u8 = 1 << 0;
pub const INPUT_ATTACK: u8 = 1 << 1;
pub const INPUT_BLOCK: u8 = 1 << 2;
pub const INPUT_SPECIAL: u8 = 1 << 3;
pub const INPUT_LEFT: u8 = 1 << 4;
pub const INPUT_RIGHT: u8 = 1 << 5;
pub fn input(_: In<PlayerHandle>, keys: Res<Input<KeyCode>>) -> u8 {
    let mut input = 0u8;
    if keys.just_pressed(KeyCode::A) {
        input |= INPUT_JUMP;
    }
    if keys.just_pressed(KeyCode::B) {
        input |= INPUT_ATTACK;
    }
    if keys.just_pressed(KeyCode::X) {
        input |= INPUT_BLOCK;
    }
    if keys.just_pressed(KeyCode::Y) {
        input |= INPUT_SPECIAL;
    }
    if keys.pressed(KeyCode::Left) {
        input |= INPUT_LEFT;
    }
    if keys.pressed(KeyCode::Right) {
        input |= INPUT_RIGHT;
    }
    input
}
