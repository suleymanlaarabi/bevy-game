use std::fs::read_to_string;

use bevy::{math::UVec2, sprite::TextureAtlasLayout};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Map {
    pub compressionlevel: i32,
    pub height: u32,
    pub width: u32,
    #[serde(rename = "tilewidth")]
    pub tile_width: u32,
    #[serde(rename = "tileheight")]
    pub tile_height: u32,
    pub infinite: bool,
    pub layers: Vec<Layer>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Layer {
    pub data: Vec<usize>,
    pub height: i32,
    pub id: i32,
    pub name: String,
    pub opacity: f32,
    #[serde(rename = "type")]
    pub layer_type: String,
    pub visible: bool,
    pub width: i32,
    pub x: i32,
    pub y: i32,
}

impl Map {
    pub fn from_string(str: String) -> Self {
      serde_json::from_str(&str)
        .expect("Maps Load Error")
    }

    pub fn from_file(path: &str) -> Self {
      Map::from_string(
        read_to_string(path)
        .expect("Unable to read map file")
      )
    }

    pub fn get_atlas(&self) -> TextureAtlasLayout {
      TextureAtlasLayout::from_grid(UVec2::new(self.tile_width, self.tile_height), self.width, self.width, None, None)
    }
}