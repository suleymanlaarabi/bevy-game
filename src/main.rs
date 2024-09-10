use bevy::app::App;
use bevy::prelude::*;
use bevy::window::WindowResolution;
use bevy_rapier2d::plugin::{NoUserData, RapierPhysicsPlugin};

use iyes_perf_ui::prelude::{PerfUiEntryFPS, PerfUiRoot};
use plugin::animation::CustomAnimationPlugin;
use plugin::debug::DebugPlugin;
use plugin::player::PlayerPlugin;
use plugin::world::WorldPlugin;

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
            WorldPlugin,
            DebugPlugin,
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.),
            PlayerPlugin,
            CustomAnimationPlugin,
        ))
        .add_systems(PreStartup, setup)
        .run();
}

#[derive(Resource, Deref, DerefMut)]
pub struct FontResource(Handle<Font>);

fn setup(mut commands: Commands, server: Res<AssetServer>) {
    let handle: Handle<Font> = server.load("arial.ttf");
    commands.insert_resource(FontResource(handle));
    commands.spawn((
        PerfUiRoot {
            display_labels: false,
            layout_horizontal: true,
            ..default()
        },
        PerfUiEntryFPS::default(),
    ));
}
