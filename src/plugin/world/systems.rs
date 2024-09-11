use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_tiled_plugin::components::{TiledCollisionSize, TiledObject};

use crate::{plugin::collision_ui::spawn_ui_with_collision, FontResource};

pub fn spawn_objects(
    mut commands: Commands,
    mut query: Query<(Entity, &TiledObject)>,
    font: Res<FontResource>,
) {
    for (entity, object) in query.iter_mut() {
        spawn_ui_with_collision(
            &mut commands,
            Vec2::new(object.position.x, object.position.y),
            Vec2::splat(100.),
            NodeBundle {
                background_color: BackgroundColor(Color::srgba(10., 10., 10., 0.08)),
                style: Style {
                    width: Val::Px(200.),
                    height: Val::Px(60.),
                    justify_self: JustifySelf::Center,
                    align_self: AlignSelf::FlexEnd,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    align_content: AlignContent::Center,
                    margin: UiRect {
                        left: Val::Px(0.),
                        right: Val::Px(0.),
                        top: Val::Px(0.),
                        bottom: Val::Px(50.),
                    },
                    border: UiRect {
                        left: Val::Px(1.),
                        right: Val::Px(1.),
                        top: Val::Px(1.),
                        bottom: Val::Px(1.),
                    },
                    ..default()
                },
                visibility: Visibility::Hidden,
                border_radius: BorderRadius::all(Val::Px(10.)),
                ..default()
            },
            |children| {
                children.spawn(TextBundle::from_section(
                    "interact E",
                    TextStyle {
                        font_size: 30.0,
                        font: font.to_owned(),
                        ..Default::default()
                    },
                ));
            },
        );
    }
}

pub fn spawn_collision(mut commands: Commands, mut query: Query<(Entity, &TiledCollisionSize)>) {
    for (entity, collision_size) in query.iter_mut() {
        commands.entity(entity).insert((
            Collider::cuboid(collision_size.x / 2., collision_size.y / 2.),
            Friction::coefficient(0.),
        ));
    }
}
