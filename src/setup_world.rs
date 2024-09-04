use bevy::prelude::*;

use crate::maps::Map;


pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (insert_tiles_image.before(setup_world), setup_world));
    }
}

#[derive(Resource)]
struct WorldResource {
  texture: Handle<Image>,
  atlas: Handle<TextureAtlasLayout>
}

fn insert_tiles_image(mut commands: Commands, server: Res<AssetServer>) {
  commands.insert_resource(WorldResource {
    texture: server.load("maps/Hills.png"),
    atlas: server.load("")
  });
}

fn sprite_bundle_from_image(image: Handle<Image>) -> SpriteBundle {
  SpriteBundle {
    texture: image.to_owned(),
    transform: Transform::from_xyz(0., 0., 0.),
    ..default()
  }
}

fn setup_world(res: Res<WorldResource>, mut commands: Commands) {
  let mut map = Map::from_file("../assets/maps/map.json");
  let mut sprites: Vec<SpriteBundle> = map.layers.get_mut(0).unwrap().data.iter_mut().map(|_| {
        let sprite_bundle = sprite_bundle_from_image(res.texture.to_owned());
        sprite_bundle
      }).collect();

  let atlas = map.get_atlas();
  let sprite = sprites.remove(0);

  commands.spawn((
    sprite
  ));
}