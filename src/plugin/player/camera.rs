use bevy::prelude::*;

use super::Player;

pub struct CameraPlayerPlugin;

impl Plugin for CameraPlayerPlugin {
    fn build(&self, app: &mut App) {
        app
          .add_systems(Startup, spawn_camera_player)
          .add_systems(Update, camera_follow_player);
    }
}

fn spawn_camera_player(
  mut commands: Commands
) {
  commands.spawn(Camera2dBundle::default());
}

fn camera_follow_player(
  mut param_set: ParamSet<(
      Query<(&Camera, &mut Transform)>,
      Query<(&Player, &Transform)>,
  )>,
) {
  let player = param_set.p1();
  let player_transform = player.single().1.translation;
  let mut camera = param_set.p0();
  let mut camera_transform = camera.single_mut().1;
  
  camera_transform.translation = player_transform;
}