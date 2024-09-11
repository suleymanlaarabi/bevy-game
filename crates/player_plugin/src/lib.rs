mod components;
mod plugins;
mod resources;
mod systems;

use bevy::prelude::*;
use plugins::camera::CameraPlayerPlugin;
use systems::*;

pub const PLAYER_SPEED: f32 = 350.0;
pub const PLAYER_Y_POS: f32 = 0.;
pub const ATTACK_DURATION: f32 = 0.8;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(CameraPlayerPlugin);
        register_systems(app);
    }
}
