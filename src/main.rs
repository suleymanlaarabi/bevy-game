use animation_plugin::CustomAnimationPlugin;
use bevy::app::App;
use bevy::prelude::*;
use bevy::window::WindowResolution;
use bevy_rapier2d::plugin::*;

use collision_event_plugin::CollisionEventPlugin;
use controls_plugin::ControlsPlugin;
use iyes_perf_ui::prelude::{PerfUiEntryFPS, PerfUiRoot};
use player_plugin::PlayerPlugin;
use plugin::debug::DebugPlugin;
use plugin::props::PropsPlugin;
use shared_resource_plugin::SharedResourcePlugin;
use world_plugin::WorldPlugin;

mod plugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "My Game".to_string(),
                        resolution: WindowResolution::new(1280., 720.),
                        ..default()
                    }),
                    ..default()
                }),
            SharedResourcePlugin,
            ControlsPlugin,
            PropsPlugin,
            DebugPlugin {
                frame: true,
                collider: false,
                ..default()
            },
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.),
            WorldPlugin,
            CollisionEventPlugin,
            PlayerPlugin,
            CustomAnimationPlugin,
        ))
        .add_systems(PreStartup, setup)
        .run();
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
