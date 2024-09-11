use bevy::prelude::*;
use controls_plugin::resources::ControlsResource;

use crate::components::Player;

pub fn handle_player_attack(
    mut query: Query<&mut Player>,
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    controls: Res<ControlsResource>,
) {
    let mut player = query.single_mut();
    if !player.is_attack && keys.pressed(controls.sword) {
        player.is_attack = true;
        player.attack_clock.reset();
        return;
    }
    player.attack_clock.tick(time.delta());
    if player.is_attack {
        if player.attack_clock.just_finished() {
            player.is_attack = false;
        }
    }
}

pub fn handle_player_shield(
    mut query: Query<&mut Player>,
    keys: Res<ButtonInput<KeyCode>>,
    controls: Res<ControlsResource>,
) {
    let mut player = query.single_mut();
    if keys.pressed(controls.shield) {
        player.is_shield = true;
        return;
    }
    player.is_shield = false;
}
