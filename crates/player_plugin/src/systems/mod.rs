mod actions;
mod animation;
mod movements;
mod setup;

use actions::*;
use animation::*;
use movements::*;
use setup::*;

use bevy::prelude::*;

pub fn register_systems(app: &mut App) {
    app.add_systems(
        Startup,
        (init_resource.before(setup_players), setup_players),
    )
    .add_systems(
        Update,
        (
            handle_player_move,
            handle_player_attack,
            check_jump_end,
            check_jump,
            handle_player_shield,
            update_player_animation,
        ),
    );
}
