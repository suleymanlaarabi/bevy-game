use bevy::prelude::*;

use crate::{core::Velocity, plugin::animation::{set_animation, AnimationBundle, AnimationIndices, AnimationTimer}};

use super::{controls::PlayerControls, resources::PlayerResource, Player, JUMP_DURATION, JUMP_FORCE, PLAYER_Y_POS};

fn spawn_player(
  commands: &mut Commands,
  player_resource: &Res<PlayerResource>,
  controls: PlayerControls
) {
  commands.spawn((
      Player::new(controls),
      SpriteBundle {
          texture: player_resource.player_sprite_image.clone(),
          transform: Transform::from_xyz(0., PLAYER_Y_POS, 0.),
          ..default()
      },
      AnimationBundle::new(0.1, 2, 8, player_resource.layout.clone()),
      Velocity::new(200., 200.)
  ));

}

pub fn handle_player_event(
  mut query: Query<(
    &Player,
    &mut Velocity,
    &mut Sprite,
    &mut AnimationIndices,
    &mut AnimationTimer
  )>, keys: Res<ButtonInput<KeyCode>>
) {    
    for (
      player,
      mut velocity,
      mut sprite,
      mut indices,
      mut timer,
    ) in &mut query {
        if keys.pressed(player.controls.right) {
            velocity.x += player.speed;
            sprite.flip_x = false;
        }
        if keys.pressed(player.controls.left) {
            velocity.x -= player.speed;
            sprite.flip_x = true;
        }
        let is_walk = velocity.x != 0.;
        if player.is_jumping {
          set_animation((&mut indices, &mut timer), 108, 114, 0.1);
          continue;
        }
        if !is_walk {
          set_animation((&mut indices, &mut timer), 0, 0, 0.1);
        } else {
          set_animation((&mut indices, &mut timer), 2, 8, 0.1);
        }
    }
}

pub fn init_resource(
  mut commands: Commands,
  asset_server: Res<AssetServer>
) {
  let player_res = PlayerResource::new(asset_server);
  println!("Player resource initialized");
  commands.insert_resource(player_res);
}

pub fn setup_players(
  mut commands: Commands,
  player_resource: Res<PlayerResource>,
) {
  spawn_player(
      &mut commands,
      &player_resource,
      PlayerControls::new(KeyCode::KeyA, KeyCode::KeyD, KeyCode::KeyW)
  );
  spawn_player(
      &mut commands,
      &player_resource,
      PlayerControls::new(KeyCode::ArrowLeft, KeyCode::ArrowRight, KeyCode::ArrowUp)
  );
}

pub fn handle_player_jump(
  time: Res<Time>,
  mut query: Query<(&mut Player, &mut Velocity, &mut Transform)>,
  keys: Res<ButtonInput<KeyCode>>
) {
  for (mut player, mut velocity, mut transform) in &mut query {
    if player.is_jumping {
      player.jump_clock.tick(time.delta());
      if player.jump_clock.elapsed_secs() < JUMP_DURATION {
        velocity.y += JUMP_FORCE;
        continue;
      }
      if transform.translation.y >= PLAYER_Y_POS {
        velocity.y -= JUMP_FORCE;
        continue;
      }
      transform.translation.y = PLAYER_Y_POS;
      player.is_jumping = false;
      continue;
    }
    if keys.pressed(player.controls.jump) {
      player.jump_clock.reset();
      player.is_jumping = true;
    }
  }
}