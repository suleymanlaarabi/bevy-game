use bevy::app::{App, Startup};
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;
use iyes_perf_ui::prelude::{PerfUiEntryFPS, PerfUiRoot};
use iyes_perf_ui::PerfUiPlugin;
use plugin::animation::CustomAnimationPlugin;
use plugin::movement::MovementPlugin;
use plugin::player::PlayerPlugin;
use setup_world::WorldPlugin;

mod core;
mod plugin;
mod maps;
mod setup_world;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            FrameTimeDiagnosticsPlugin,
            PerfUiPlugin,
            WorldPlugin,
            PlayerPlugin,
            MovementPlugin,
            CustomAnimationPlugin,
        ))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        PerfUiRoot {
            display_labels: false,
            layout_horizontal: true,
            ..default()
        },
        PerfUiEntryFPS::default(),
    ));

}

