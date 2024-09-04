use bevy::prelude::*;

#[derive(Resource)]
pub struct PlayerResource {
  pub player_sprite_image: Handle<Image>,
  pub layout: Handle<TextureAtlasLayout>
}

impl PlayerResource {
  pub fn new(asset_server: Res<AssetServer>) -> Self {
    let player_sprite_image = asset_server.load("sprites.png");
    let layout = asset_server.add(
      TextureAtlasLayout::from_grid(UVec2::splat(80), 12, 12, None, None)
    );
    Self {
      player_sprite_image,
      layout
    }
  }
}