use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*};
use bevy_rapier2d::render::RapierDebugRenderPlugin;
use iyes_perf_ui::PerfUiPlugin;
pub struct DebugPlugin {
    pub frame: bool,
    pub collider: bool,
}

impl Default for DebugPlugin {
    fn default() -> Self {
        DebugPlugin {
            frame: false,
            collider: false,
        }
    }
}

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        if self.frame {
            app.add_plugins((FrameTimeDiagnosticsPlugin, PerfUiPlugin));
        }
        if self.collider {
            app.add_plugins((RapierDebugRenderPlugin::default(),));
        }
    }
}
