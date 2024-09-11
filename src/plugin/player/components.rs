use bevy::prelude::*;

use super::{controls::PlayerControls, ATTACK_DURATION};

#[derive(Component)]
pub struct Jump;

#[derive(Component, Debug)]
pub struct Player {
    pub controls: PlayerControls,
    pub is_jumping: bool,
    pub is_attack: bool,
    pub attack_clock: Timer,
    pub is_shield: bool,
}
#[derive(Component)]

pub struct PlayerCollider;

impl Player {
    pub fn new(controls: PlayerControls) -> Self {
        Self {
            is_jumping: false,
            is_attack: false,
            is_shield: false,
            attack_clock: Timer::from_seconds(ATTACK_DURATION, TimerMode::Once),
            controls,
        }
    }
}
