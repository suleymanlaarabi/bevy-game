use bevy::prelude::*;

pub fn interact_rect() -> NodeBundle {
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
    }
}
