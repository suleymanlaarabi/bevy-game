use bevy::prelude::*;

#[derive(Debug)]
pub struct PlayerControls {
    pub left: KeyCode,
    pub right: KeyCode,
    pub jump: KeyCode,
    pub attack: KeyCode,
    pub shield: KeyCode,
}

impl PlayerControls {
    pub fn new(
        left: KeyCode,
        right: KeyCode,
        jump: KeyCode,
        attack: KeyCode,
        shield: KeyCode,
    ) -> Self {
        Self {
            left,
            right,
            jump,
            attack,
            shield,
        }
    }
}
