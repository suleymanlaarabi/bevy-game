use bevy::prelude::*;

#[derive(Resource, Deref, DerefMut)]
pub struct FontResource(pub Handle<Font>);
pub struct SharedResourcePlugin;

impl Plugin for SharedResourcePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(PreStartup, insert_default_resources);
    }
}

fn insert_default_resources(mut commands: Commands, server: Res<AssetServer>) {
    commands.insert_resource(FontResource(server.load("arial.ttf")));
}
