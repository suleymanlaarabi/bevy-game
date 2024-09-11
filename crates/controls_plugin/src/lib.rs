pub mod resources;

use bevy::prelude::*;
use resources::ControlsResource;
pub struct ControlsPlugin;

impl Plugin for ControlsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ControlsResource::default());
    }
}
