use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::plugin::player::{
    components::{Jump, Player, PlayerCollider},
    PLAYER_SPEED,
};

pub fn handle_player_move(
    mut query: Query<(&mut Player, &mut Sprite, &mut Velocity)>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    let (player, mut sprite, mut velocity) = query.single_mut();
    velocity.linvel.x = 0.;
    if player.is_shield {
        return;
    }

    if keys.pressed(player.controls.right) && !player.is_attack {
        velocity.linvel.x = PLAYER_SPEED;
        sprite.flip_x = false;
    }
    if keys.pressed(player.controls.left) && !player.is_attack {
        velocity.linvel.x = -PLAYER_SPEED;
        sprite.flip_x = true;
    }
}

pub fn check_jump(
    keys: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut Player, &mut Velocity), (With<Player>, Without<Jump>)>,
) {
    if query.is_empty() {
        return;
    }

    let (entity, mut player, mut velocity) = query.single_mut();

    if !player.is_shield && keys.pressed(player.controls.jump) {
        player.is_jumping = true;
        commands.entity(entity).insert(Jump);
        velocity.linvel.y = 450.;
    }
}

pub fn check_jump_end(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Player), (With<Player>, With<Jump>)>,
    colliders_with_player: Query<&Collider, With<PlayerCollider>>,
    colliders: Query<&Collider, With<Sensor>>,
    mut contact_events: EventReader<CollisionEvent>,
) {
    for (entity, mut player) in query.iter_mut() {
        for collision_event in contact_events.read() {
            if let CollisionEvent::Started(h1, h2, _) = collision_event {
                let is_sensor1 = colliders.get(*h1).is_ok();
                let is_sensor2 = colliders.get(*h2).is_ok();
                if is_sensor1 && is_sensor2 {
                    continue;
                }
                let is_player1 = colliders_with_player.get(*h1).is_ok();
                let is_player2 = colliders_with_player.get(*h2).is_ok();
                if is_player1 || is_player2 {
                    player.is_jumping = false;
                    commands.entity(entity).remove::<Jump>();
                }
            }
        }
    }
}
