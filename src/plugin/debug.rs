use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*};
use bevy_rapier2d::render::RapierDebugRenderPlugin;
use iyes_perf_ui::PerfUiPlugin;
pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            FrameTimeDiagnosticsPlugin,
            PerfUiPlugin,
            RapierDebugRenderPlugin::default(),
        ));
    }
}
