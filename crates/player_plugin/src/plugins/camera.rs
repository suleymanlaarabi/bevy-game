use bevy::prelude::*;

use crate::components::Player;

pub struct CameraPlayerPlugin;

impl Plugin for CameraPlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera_player)
            .add_systems(Update, camera_follow_player);
    }
}

fn spawn_camera_player(mut commands: Commands) {
    let mut cam = Camera2dBundle::default();
    cam.transform.scale = Vec3::splat(1.);
    commands.spawn(cam);
}

fn camera_follow_player(
    mut param_set: ParamSet<(
        Query<(&Camera, &mut Transform)>,
        Query<(&Player, &Transform)>,
    )>,
    time: Res<Time>,
) {
    param_set.p0().single_mut().1.translation = param_set.p0().single_mut().1.translation.lerp(
        param_set.p1().single().1.translation,
        5.0 * time.delta_seconds(),
    );
}
