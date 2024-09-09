use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::plugin::player::{
    components::{Jump, Player},
    PLAYER_SPEED,
};

pub fn handle_player_move(
    mut query: Query<(&mut Player, &mut Sprite, &mut Transform, &mut Velocity)>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    let (player, mut sprite, mut transform, mut velocity) = query.single_mut();

    if keys.pressed(player.controls.right) && !player.is_attack {
        velocity.linvel.x = PLAYER_SPEED;
        sprite.flip_x = false;
    }
    if keys.pressed(player.controls.left) && !player.is_attack {
        velocity.linvel.x = -PLAYER_SPEED;
        sprite.flip_x = true;
    }
    transform.translation.y += 0.01;
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
    mut contact_events: EventReader<CollisionEvent>,
) {
    for (entity, mut player) in query.iter_mut() {
        for collision_event in contact_events.read() {
            if let CollisionEvent::Started(_, _, _) = collision_event {
                player.is_jumping = false;
                commands.entity(entity).remove::<Jump>();
            }
        }
    }
}
