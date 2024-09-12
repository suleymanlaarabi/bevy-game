use bevy::prelude::*;
use shared_resource_plugin::FontResource;

#[derive(Component)]
struct FPSNode;

pub fn fps_displayer_build(app: &mut App) {
    app.add_systems(Startup, spawn_fps_node)
        .add_systems(Update, display_fps_system);
}

fn spawn_fps_node(mut commands: Commands, font: Res<FontResource>) {
    commands
        .spawn((NodeBundle {
            background_color: BackgroundColor(Color::srgba(10., 10., 10., 0.08)),
            style: Style {
                width: Val::Px(200.),
                height: Val::Px(60.),
                justify_self: JustifySelf::End,
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
        },))
        .with_children(|child| {
            child.spawn((
                TextBundle::from_section(
                    "interact E",
                    TextStyle {
                        font_size: 30.0,
                        font: font.to_owned(),
                        ..Default::default()
                    },
                ),
                FPSNode,
            ));
        });
}

fn display_fps_system(time: Res<Time>, mut query: Query<&mut Text, With<FPSNode>>) {
    let fps = (1.0 / time.delta_seconds()) as u32;
    for mut text in query.iter_mut() {
        let buffer = &mut text.sections[0].value;
        buffer.clear();
        buffer.push_str(fps.to_string().as_str());
    }
}
