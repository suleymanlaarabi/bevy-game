use bevy::prelude::*;

#[derive(Resource)]
pub struct ControlsResource {
    pub move_right: KeyCode,
    pub move_left: KeyCode,
    pub move_up: KeyCode,
    pub shield: KeyCode,
    pub sword: KeyCode,
    pub interact: KeyCode,
}

impl Default for ControlsResource {
    fn default() -> Self {
        ControlsResource {
            move_right: KeyCode::KeyD,
            move_left: KeyCode::KeyA,
            move_up: KeyCode::Space,
            sword: KeyCode::KeyN,
            shield: KeyCode::KeyB,
            interact: KeyCode::KeyE,
        }
    }
}
