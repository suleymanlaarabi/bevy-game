use bevy::prelude::*;

use crate::plugin::player::components::Player;

pub fn handle_player_attack(
    mut query: Query<&mut Player>,
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
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

pub fn handle_player_shield(mut query: Query<&mut Player>, keys: Res<ButtonInput<KeyCode>>) {
    let mut player = query.single_mut();
    if keys.pressed(player.controls.shield) {
        player.is_shield = true;
        return;
    }
    player.is_shield = false;
}
