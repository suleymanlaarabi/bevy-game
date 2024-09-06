mod controls;
mod systems;
mod resources;
mod camera;

use bevy::prelude::*;
use camera::CameraPlayerPlugin;
use controls::PlayerControls;
use systems::{handle_player_attack, handle_player_event, handle_player_jump, init_resource, setup_players};

pub const JUMP_DURATION: f32 = 0.35;
pub const JUMP_FORCE: f32 = 450.0;
pub const PLAYER_SPEED: f32 = 300.0;
pub const PLAYER_Y_POS: f32 = 0.0;
pub const ATTACK_DURATION: f32 = 0.8;


#[derive(Component)]
struct Player {
  speed: f32,
  controls: PlayerControls,
  is_jumping: bool,
  jump_clock: Timer,
  is_attack: bool,
  attack_clock: Timer
}

impl Player {
  pub fn new(
    controls: PlayerControls
  ) -> Self {
    Self {
      speed: PLAYER_SPEED,
      is_jumping: false,
      jump_clock: Timer::from_seconds(JUMP_DURATION, TimerMode::Once),
      is_attack: false,
      attack_clock: Timer::from_seconds(ATTACK_DURATION, TimerMode::Once),
      controls
    }
  }
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_plugins(CameraPlayerPlugin)
        .add_systems(Startup, (init_resource.before(setup_players), setup_players))
        .add_systems(Update, (handle_player_event, handle_player_attack, handle_player_jump));
    }
}




