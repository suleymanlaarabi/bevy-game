use bevy::prelude::*;

use crate::core::Velocity;

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Update, check_movement);
    }
}

fn check_movement(
  time: Res<Time>,
  mut query: Query<(&mut Velocity, &mut Transform)>
) {
    let delta = time.delta_seconds();
    for (mut velocity, mut transform) in &mut query {
        let new_x = transform.translation.x + velocity.x * delta;
        let new_y = transform.translation.y + velocity.y * delta;

        transform.translation.y = new_y;
        transform.translation.x = new_x;
        velocity.x = 0.;
        velocity.y = 0.;
    }
}