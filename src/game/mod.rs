use crate::AppState;
use bevy::prelude::*;
use enemy::EnemyPlugin;
use glimpse::GlimpsePlugin;
use leafwing_input_manager::prelude::*;
use map::MapPlugin;
use player::{resources::PlayerAction, PlayerPlugin};
use systems::*;

mod enemy;
mod glimpse;
mod map;
mod player;
mod systems;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            EnemyPlugin,
            GlimpsePlugin,
            MapPlugin,
            PlayerPlugin,
            InputManagerPlugin::<PlayerAction>::default(),
        ))
        .add_systems(
            OnEnter(AppState::Game),
            (spawn_map, spawn_player, spawn_glimpses, spawn_enemies).chain(),
        );
    }
}
