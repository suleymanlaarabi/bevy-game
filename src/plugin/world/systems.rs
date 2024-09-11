use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_tiled_plugin::components::{TiledCollisionSize, TiledObject};

use crate::FontResource;

use super::components::{WorldInteract, WorldInteractSpawned, WorldInteractUI};

pub fn spawn_objects(mut commands: Commands, mut query: Query<(Entity, &TiledObject)>) {
    for (entity, _) in query.iter_mut() {
        let bundle: (WorldInteract, Collider, Sensor, ActiveEvents) = (
            WorldInteract::new(KeyCode::KeyA, "Hello world"),
            Collider::cuboid(100., 100.),
            Sensor::default(),
            ActiveEvents::COLLISION_EVENTS,
        );
        commands.entity(entity).insert(bundle);
    }
}

pub fn handle_ui_interaction(
    mut commands: Commands,
    mut query: Query<(
        Entity,
        &Transform,
        &WorldInteract,
        Option<&WorldInteractSpawned>,
    )>,
    query_ui: Query<Entity, With<WorldInteractUI>>,
    mut contact_events: EventReader<CollisionEvent>,
    font: Res<FontResource>,
) {
    for collision in contact_events.read() {
        match collision {
            CollisionEvent::Started(h1, h2, _) => {
                for (entity, _, _, ui_spawned) in query.iter_mut() {
                    if h1 == &entity || h2 == &entity {
                        if ui_spawned.is_none() {
                            commands.entity(entity).insert(WorldInteractSpawned);
                            build_object_ui(&mut commands, &font);
                        }
                    }
                }
            }
            CollisionEvent::Stopped(h1, h2, _) => {
                for (entity, _, _, ui_spawned) in query.iter_mut() {
                    if h1 == &entity || h2 == &entity {
                        if ui_spawned.is_some() {
                            commands.entity(entity).remove::<WorldInteractSpawned>();
                            for ui in query_ui.iter() {
                                commands.entity(ui).despawn_recursive();
                            }
                        }
                    }
                }
            }
        }
    }
}

fn build_object_ui(commands: &mut Commands, font: &Res<FontResource>) -> impl Bundle {
    let default_bundle = (
        WorldInteractUI,
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
            border_radius: BorderRadius::all(Val::Px(10.)),

            ..default()
        },
    );
    commands.spawn(default_bundle).with_children(|children| {
        children.spawn(TextBundle::from_section(
            "interact E",
            TextStyle {
                font: font.0.clone(),
                font_size: 30.0,

                ..default()
            },
        ));
    });
}

pub fn spawn_collision(mut commands: Commands, mut query: Query<(Entity, &TiledCollisionSize)>) {
    for (entity, collision_size) in query.iter_mut() {
        commands.entity(entity).insert((
            Collider::cuboid(collision_size.x / 2., collision_size.y / 2.),
            Friction::coefficient(0.),
        ));
    }
}
