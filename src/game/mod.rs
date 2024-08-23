use crate::AppState;
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;
use player::{resources::PlayerAction, PlayerPlugin};
use pooling::ObjectPool;
use systems::*;

mod components;
mod map_gen;
mod player;
mod pooling;
mod systems;

pub const FLOOR_AMOUNT: usize = 210;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((PlayerPlugin, InputManagerPlugin::<PlayerAction>::default()))
            .insert_resource(ObjectPool::default())
            .add_systems(
                OnEnter(AppState::Game),
                (create_map, create_glimpses, create_player).chain(),
            )
            .add_systems(
                Update,
                (update_floors, update_glimpses).run_if(in_state(AppState::Game)),
            );
    }
}
