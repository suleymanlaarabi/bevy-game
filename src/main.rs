use bevy::app::App;
use bevy::prelude::*;
use bevy::window::WindowResolution;
use bevy_rapier2d::plugin::*;

use bevy_rapier2d::prelude::{ActiveEvents, Collider, CollisionEvent, Sensor};
use iyes_perf_ui::prelude::{PerfUiEntryFPS, PerfUiRoot};
use plugin::animation::CustomAnimationPlugin;
use plugin::collision_ui::{spawn_ui_with_collision, CollisionUI, CollisionUIPlugin};
use plugin::debug::DebugPlugin;
use plugin::player::PlayerPlugin;
use plugin::world::WorldPlugin;

mod plugin;
mod ui;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "My Game".to_string(),
                        resolution: WindowResolution::new(1280., 720.),
                        ..default()
                    }),
                    ..default()
                }),
            CollisionUIPlugin,
            WorldPlugin,
            DebugPlugin,
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.),
            PlayerPlugin,
            CustomAnimationPlugin,
        ))
        .add_systems(PreStartup, setup)
        .run();
}

#[derive(Resource, Deref, DerefMut)]
pub struct FontResource(Handle<Font>);

fn setup(mut commands: Commands, server: Res<AssetServer>) {
    let handle: &Handle<Font> = &server.load("arial.ttf");
    spawn_ui_with_collision(
        &mut commands,
        Vec2::new(500., -800.),
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
                    font: handle.to_owned(),
                    ..Default::default()
                },
            ));
        },
    );
    commands.insert_resource(FontResource(handle.to_owned()));
    commands.spawn((
        PerfUiRoot {
            display_labels: false,
            layout_horizontal: true,
            ..default()
        },
        PerfUiEntryFPS::default(),
    ));
}
