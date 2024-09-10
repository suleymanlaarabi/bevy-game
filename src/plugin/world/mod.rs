mod components;
mod systems;

use bevy::app::{Plugin, PostStartup, Update};
use bevy_tiled_plugin::default_plugin::TiledPlugin;
use systems::{handle_ui_interaction, spawn_collision, spawn_objects};

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(TiledPlugin::from_json(
            "assets/maps/default/map.json",
            "assets/maps/default/tiles.json",
            "maps/default/images/Tiles.png",
            4.,
        ))
        .add_systems(PostStartup, (spawn_objects, spawn_collision))
        .add_systems(Update, handle_ui_interaction);
    }
}
