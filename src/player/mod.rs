use crate::GameState;
use bevy::prelude::*;
use resources::PlayerInput;
use systems::*;

pub mod bundles;
pub mod components;
pub mod resources;
mod systems;

pub const ANGLE_EPSILON: f32 = 0.001953125;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerInput>().add_systems(
            Update,
            (
                player_input,
                player_move,
                player_look,
                player_floor,
                player_death,
            )
                .run_if(in_state(GameState::Game)),
        );
    }
}
