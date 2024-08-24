use crate::AppState;
use bevy::prelude::*;
use resources::ObjectPool;
use systems::*;

pub mod components;
pub mod resources;
pub mod systems;

pub const FLOOR_AMOUNT: usize = 210;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ObjectPool::default())
            .add_systems(OnEnter(AppState::Game), spawn_map)
            .add_systems(Update, update_floors.run_if(in_state(AppState::Game)));
    }
}
