use bevy::prelude::{Query, With};
use bevy_rapier2d::prelude::Velocity;

use crate::plugin::{
    animation::{set_animation, AnimationIndices, AnimationTimer},
    player::components::Player,
};

pub fn update_player_animation(
    mut query: Query<
        (
            &Velocity,
            &Player,
            &mut AnimationIndices,
            &mut AnimationTimer,
        ),
        With<Player>,
    >,
) {
    if query.is_empty() {
        return;
    }
    let (velocity, player, indices, timer) = &mut query.single_mut();

    if player.is_jumping {
        set_animation((indices, timer), 24, 38, 0.1);
        return;
    }
    if player.is_shield {
        set_animation((indices, timer), 81, 82, 0.3);
        return;
    }
    if player.is_attack {
        set_animation((indices, timer), 8, 16, 0.1);
        return;
    }
    if velocity.linvel.x <= -0.7 || velocity.linvel.x >= 0.7 {
        set_animation((indices, timer), 16, 23, 0.1);
    } else {
        set_animation((indices, timer), 0, 5, 0.1);
    }
}
