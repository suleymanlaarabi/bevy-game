use bevy::prelude::*;

#[derive(Component)]
pub struct WorldInteractSpawned;

#[derive(Component)]
pub struct WorldInteractIn;

#[derive(Component)]
pub struct WorldInteract {
    pub touch: KeyCode,
    pub text: String,
}

impl WorldInteract {
    pub fn new(touch: KeyCode, text: &str) -> Self {
        Self {
            touch,
            text: text.to_string(),
        }
    }
}
