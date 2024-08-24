use bevy::prelude::*;

#[derive(Component)]
pub struct Enemy {
    pub speed: f32,
}

#[derive(Component)]
pub struct EnemyAnimations {
    pub animations: Vec<AnimationNodeIndex>,
    #[allow(dead_code)]
    pub graph: Handle<AnimationGraph>,
}

impl Default for Enemy {
    fn default() -> Self {
        Self { speed: 0.01 }
    }
}
