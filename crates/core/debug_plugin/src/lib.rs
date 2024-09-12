use bevy::prelude::*;
use fps_displayer::fps_displayer_build;

mod fps_displayer;
pub struct DebugPlugin {
    pub fps: bool,
}

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        fps_displayer_build(app);
    }
}

impl Default for DebugPlugin {
    fn default() -> Self {
        Self { fps: false }
    }
}
