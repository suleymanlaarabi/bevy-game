use bevy::prelude::*;

pub struct PlayerControls {
  pub left: KeyCode,
  pub right: KeyCode,
  pub jump: KeyCode,
  pub attack: KeyCode
}

impl PlayerControls {
  pub fn new(
    left: KeyCode,
    right: KeyCode,
    jump: KeyCode,
    attack: KeyCode
  ) -> Self {
    Self {
      left,
      right,
      jump,
      attack
    }
  }
}