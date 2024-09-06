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
          sprite: Sprite {
            custom_size: Some(Vec2::new(150., 150.)),
            ..default()
          },
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
        if keys.pressed(player.controls.right) && !player.is_attack {
            velocity.x += player.speed;
            sprite.flip_x = false;
        }
        if keys.pressed(player.controls.left) && !player.is_attack {
            velocity.x -= player.speed;
            sprite.flip_x = true;
        }
        let is_walk = velocity.x != 0.;
        if player.is_attack {
          set_animation((&mut indices, &mut timer), 8, 16, 0.1);
          continue;
        }
        if player.is_jumping {
          set_animation((&mut indices, &mut timer), 24, 38, 0.1);
          continue;
        }
        if !is_walk {
          set_animation((&mut indices, &mut timer), 0, 5, 0.1);
        } else {
          set_animation((&mut indices, &mut timer), 16, 23, 0.1);
        }
    }
}

pub fn handle_player_attack(
  mut query: Query<&mut Player>,
  keys: Res<ButtonInput<KeyCode>>,
  time: Res<Time>
) {
  let mut player = query.single_mut();
  player.attack_clock.tick(time.delta());
  if !player.is_attack && keys.pressed(player.controls.attack) {
    player.is_attack = true;
    player.attack_clock.reset();
    return;
  }
  if player.is_attack {
    if player.attack_clock.just_finished() {
      player.is_attack = false;
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
      PlayerControls::new(
        KeyCode::KeyA,
        KeyCode::KeyD,
        KeyCode::KeyW,
        KeyCode::KeyN
      )
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
    if keys.pressed(player.controls.jump) && !player.is_attack {
      player.jump_clock.reset();
      player.is_jumping = true;
    }
  }
}