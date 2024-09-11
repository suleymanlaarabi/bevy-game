pub mod components;
pub mod systems;

use bevy::{
    app::{Plugin, PostStartup, Update},
    prelude::IntoSystemConfigs,
};
use bevy_tiled_plugin::default_plugin::TiledPlugin;
use collision_event_plugin::detect_collision;
use systems::{in_enter, on_enter, spawn_collision, spawn_objects};

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
        .add_systems(Update, (on_enter.after(detect_collision), in_enter));
    }
}
