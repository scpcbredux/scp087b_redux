use std::time::Duration;

use bevy::prelude::*;

#[derive(Component)]
pub struct Player {
    pub kill_timer: f32,
    pub floor_index: usize,
    pub camera_height: Vec3,
    pub mouse_sensitivity: f32,
    pub speed: f32,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            kill_timer: 0.0,
            floor_index: 1,
            camera_height: Vec3::Y * 0.7,
            mouse_sensitivity: 0.003,
            speed: 2.0,
        }
    }
}

#[derive(Component)]
pub struct PlayerFootsteps {
    pub timer: Timer,
}

impl Default for PlayerFootsteps {
    fn default() -> Self {
        Self {
            timer: Timer::new(Duration::from_secs_f32(2.0), TimerMode::Repeating),
        }
    }
}

#[derive(Component)]
pub struct PlayerCamera {
    pub speed: f32,
    pub max_bob: Vec3,
    pub tilt: f32,
    pub timer: f32,
}

impl Default for PlayerCamera {
    fn default() -> Self {
        Self {
            speed: 7.0,
            max_bob: Vec3::splat(0.07),
            tilt: 0.5f32.to_radians(),
            timer: 0.0,
        }
    }
}
