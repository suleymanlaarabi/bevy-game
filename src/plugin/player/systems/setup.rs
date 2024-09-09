use bevy::prelude::*;

use crate::plugin::player::*;

pub fn spawn_player(
    commands: &mut Commands,
    player_resource: &Res<PlayerResource>,
    controls: PlayerControls,
) {
    commands
        .spawn((
            Player::new(controls),
            SpriteBundle {
                texture: player_resource.player_sprite_image.clone(),
                transform: Transform::from_xyz(50., PLAYER_Y_POS, 3.),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(150., 150.)),
                    ..default()
                },
                ..default()
            },
            AnimationBundle::new(0.1, 2, 8, player_resource.layout.clone()),
            RigidBody::Dynamic,
            LockedAxes::ROTATION_LOCKED,
            Velocity::default(),
        ))
        .with_children(|children| {
            children.spawn((
                Collider::cuboid(46., 50.),
                Transform::from_xyz(0., -27., 0.),
                ActiveEvents::COLLISION_EVENTS,
            ));
        });
}

pub fn setup_players(mut commands: Commands, player_resource: Res<PlayerResource>) {
    spawn_player(
        &mut commands,
        &player_resource,
        PlayerControls::new(
            KeyCode::KeyA,
            KeyCode::KeyD,
            KeyCode::KeyW,
            KeyCode::KeyN,
            KeyCode::KeyB,
        ),
    );
}

pub fn init_resource(mut commands: Commands, asset_server: Res<AssetServer>) {
    let player_res = PlayerResource::new(asset_server);
    println!("Player resource initialized");
    commands.insert_resource(player_res);
}
