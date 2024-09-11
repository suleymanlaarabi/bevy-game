use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_tiled_plugin::components::{TiledCollisionSize, TiledObject};

use crate::{
    plugin::{
        collision_ui::{spawn_ui_with_collision, CollisionUIEvent},
        controls::resources::ControlsResource,
    },
    ui::interact_rect,
    FontResource,
};

use super::components::{WorldInteract, WorldInteractIn};

pub fn on_enter(
    query: Query<(Entity, &WorldInteract), With<WorldInteract>>,
    mut ev_rd: EventReader<CollisionUIEvent>,
    mut commands: Commands,
) {
    for ev in ev_rd.read() {
        if let Ok((entity, _)) = query.get(ev.entity) {
            if !ev.is_exit {
                commands.entity(entity).insert(WorldInteractIn);
            } else {
                commands.entity(entity).remove::<WorldInteractIn>();
            }
        }
    }
}

pub fn in_enter(
    query: Query<Entity, With<WorldInteractIn>>,
    controls: Res<ControlsResource>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    query.iter().for_each(|_| {
        if keys.just_pressed(controls.interact) {
            println!("interact");
        }
    })
}

pub fn spawn_objects(
    mut commands: Commands,
    mut query: Query<&TiledObject>,
    font: Res<FontResource>,
) {
    for object in query.iter_mut() {
        spawn_ui_with_collision(
            &mut commands,
            Vec2::new(object.position.x, object.position.y),
            Vec2::splat(100.),
            (WorldInteract::new(KeyCode::KeyE, "coucou"), interact_rect()),
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
