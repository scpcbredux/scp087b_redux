use bevy::prelude::*;

#[derive(Component)]
pub struct Enemy {
    pub speed: f32,
}

impl Default for Enemy {
    fn default() -> Self {
        Self { speed: 0.01 }
    }
}
