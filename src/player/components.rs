use std::time::Duration;

use bevy::prelude::*;

#[derive(Component)]
pub struct Player {
    pub kill_timer: f32,
    pub floor_index: usize,
    pub camera_height: Vec3,
    pub mouse_sensitivity: f32,
    pub footstep_timer: Timer,
}

#[derive(Component)]
pub struct PlayerCamera;

impl Default for Player {
    fn default() -> Self {
        Self {
            kill_timer: 0.0,
            floor_index: 1,
            camera_height: Vec3::Y * 0.5,
            mouse_sensitivity: 0.003,
            footstep_timer: Timer::new(Duration::from_secs_f32(2.0), TimerMode::Repeating),
        }
    }
}
