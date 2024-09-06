use std::fs::read_to_string;

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Resource, Serialize, Deserialize, Debug)]
pub struct World {
    height: u32,
    width: u32,
    tile_height: u32,
    tile_width: u32,
    layers: Vec<Vec<u16>>,
    collision_layers: Vec<Vec<u16>>
}

pub struct WorldPlugin {
    world_path: String
}

impl World {
    pub fn create_from_string(str: String) -> Self {
        serde_json::from_str(&str).expect("Unable to parse world")
    }
    pub fn create_from_file(path: &String) -> Self {
       World::create_from_string(read_to_string(path).expect("Unable to read file"))
    }
}

impl WorldPlugin {
    pub fn new(
        world_path: &str
    ) -> Self {
        Self {
            world_path: world_path.to_string()
        }
    }
}

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(World::create_from_file(&self.world_path))
            .add_systems(Startup, (insert_world_resource.before(spawn_world), spawn_world));
    }
}

#[derive(Resource)]
struct WorldResource {
    atlas: Handle<TextureAtlasLayout>,
    texture: Handle<Image>
}

fn insert_world_resource(mut commands: Commands, server: Res<AssetServer>, world: Res<World>) {
    let atlas: Handle<TextureAtlasLayout> = server.add(
        TextureAtlasLayout::from_grid(UVec2::new(world.tile_width, world.tile_height), world.width, world.height, None, None)
    );
    let texture: Handle<Image> = server.load("tiles/Hills.png");
    commands.insert_resource(WorldResource {
        atlas,
        texture
    });
}

fn create_world_element(
    res: &Res<WorldResource>,
    word: &Res<World>,
    offset: usize,
    row: usize,
    column: usize
) -> (SpriteBundle, TextureAtlas) {
    (
        SpriteBundle {
            texture: res.texture.clone(),
            transform: Transform::from_xyz(
                (column * word.tile_width as usize) as f32,
                (row * word.tile_width as usize) as f32,
                0.
            ),
            ..default()
        },
        TextureAtlas {
            layout: res.atlas.clone(),
            index: offset
        }
    )
}

fn spawn_world(mut commands: Commands, world_resource: Res<WorldResource>, world: Res<World>) {
    for row in &world.layers[0..0] {
        for (index, element) in row.iter().enumerate() {
            commands.spawn(
                create_world_element(
                    &world_resource,
                    &world,
                    element.to_owned() as usize,
                    1,
                    index
                )
            );
        }
    }
}