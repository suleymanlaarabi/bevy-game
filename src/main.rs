use bevy::app::{App, Startup};
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;
use iyes_perf_ui::prelude::{PerfUiEntryFPS, PerfUiRoot};
use iyes_perf_ui::PerfUiPlugin;
use plugin::animation::CustomAnimationPlugin;
use plugin::movement::MovementPlugin;
use plugin::player::PlayerPlugin;
use plugin::world::WorldPlugin;

mod core;
mod plugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
            .set(ImagePlugin::default_nearest())
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Bevy game".to_string(),
                    ..default()
                }),
                ..default()
            }),
            FrameTimeDiagnosticsPlugin,
            PerfUiPlugin,
            WorldPlugin::new("./assets/maps/map.json"),
            PlayerPlugin,
            MovementPlugin,
            CustomAnimationPlugin,
        ))
        .add_systems(Startup, setup)
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

