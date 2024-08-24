use crate::AppState;
use bevy::{animation::animate_targets, prelude::*};
use systems::*;

pub mod components;
mod systems;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            enemies_once_loaded
                .before(animate_targets)
                .run_if(in_state(AppState::Game)),
        )
        .add_systems(
            Update,
            (enemies_update, enemies_animation).run_if(in_state(AppState::Game)),
        );
    }
}
