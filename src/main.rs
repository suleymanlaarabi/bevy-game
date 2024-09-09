use bevy::app::{App, Startup};
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;
use bevy::window::WindowResolution;
use bevy_rapier2d::plugin::{NoUserData, RapierPhysicsPlugin};
use bevy_rapier2d::prelude::*;
use bevy_tiled_plugin::default_plugin::{TiledCollisionSize, TiledPlugin};
use iyes_perf_ui::prelude::{PerfUiEntryFPS, PerfUiRoot};
use iyes_perf_ui::PerfUiPlugin;
use plugin::animation::CustomAnimationPlugin;
use plugin::player::PlayerPlugin;

mod plugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Bevy game".to_string(),
                        resolution: WindowResolution::new(1280., 720.),
                        ..default()
                    }),
                    ..default()
                }),
            FrameTimeDiagnosticsPlugin,
            PerfUiPlugin,
            TiledPlugin::from_json(
                "assets/maps/map.json",
                "assets/maps/hills.json",
                "maps/images/Hills.png",
                4.,
            )
            .set_position(Vec2::new(600., -500.)),
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.),
            PlayerPlugin,
            CustomAnimationPlugin,
        ))
        .add_systems(Startup, setup)
        .add_systems(PostStartup, spawn_collision)
        .run();
}

fn spawn_collision(mut commands: Commands, mut query: Query<(Entity, &TiledCollisionSize)>) {
    for (entity, collision_size) in query.iter_mut() {
        commands.entity(entity).insert((
            RigidBody::Fixed,
            Collider::cuboid(collision_size.x / 2., collision_size.y / 2.),
        ));
    }
}

fn setup(mut commands: Commands) {
    commands.spawn((
        PerfUiRoot {
            display_labels: false,
            layout_horizontal: true,
            ..default()
        },
        PerfUiEntryFPS::default(),
    ));
}
