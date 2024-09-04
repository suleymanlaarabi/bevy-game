mod controls;
mod systems;
mod resources;

use bevy::prelude::*;
use controls::PlayerControls;
use systems::{handle_player_event, handle_player_jump, init_resource, setup_players};

pub const JUMP_DURATION: f32 = 0.27;
pub const JUMP_FORCE: f32 = 450.0;
pub const PLAYER_SPEED: f32 = 300.0;
pub const PLAYER_Y_POS: f32 = 0.0;


#[derive(Component)]
struct Player {
  speed: f32,
  controls: PlayerControls,
  is_jumping: bool,
  jump_clock: Timer
}

impl Player {
  pub fn new(
    controls: PlayerControls
  ) -> Self {
    Self {
      speed: PLAYER_SPEED,
      is_jumping: false,
      jump_clock: Timer::from_seconds(JUMP_DURATION, TimerMode::Once),
      controls: PlayerControls {
        left: controls.left,
        right: controls.right,
        jump: controls.jump
      }
    }
  }
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Startup, (init_resource.before(setup_players), setup_players))
        .add_systems(Update, (handle_player_event, handle_player_jump));
    }
}




