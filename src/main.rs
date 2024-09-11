use bevy::app::App;
use bevy::prelude::*;
use bevy::window::WindowResolution;
use bevy_rapier2d::plugin::*;

use iyes_perf_ui::prelude::{PerfUiEntryFPS, PerfUiRoot};
use plugin::animation::CustomAnimationPlugin;
use plugin::collision_ui::CollisionUIPlugin;
use plugin::controls::ControlsPlugin;
use plugin::debug::DebugPlugin;
use plugin::player::PlayerPlugin;
use plugin::world::WorldPlugin;

mod plugin;
mod ui;

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
            ControlsPlugin,
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.),
            WorldPlugin,
            CollisionUIPlugin,
            PlayerPlugin,
            CustomAnimationPlugin,
        ))
        .add_systems(PreStartup, setup)
        .run();
}

#[derive(Resource, Deref, DerefMut)]
pub struct FontResource(Handle<Font>);

fn setup(mut commands: Commands, server: Res<AssetServer>) {
    let handle: &Handle<Font> = &server.load("arial.ttf");

    commands.insert_resource(FontResource(handle.to_owned()));
    commands.spawn((
        PerfUiRoot {
            display_labels: false,
            layout_horizontal: true,
            ..default()
        },
        PerfUiEntryFPS::default(),
    ));
}
