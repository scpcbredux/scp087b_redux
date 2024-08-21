use avian3d::prelude::*;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_mod_billboard::prelude::*;
use bevy_rand::prelude::*;
use game::GamePlugin;
use resources::{AudioAssets, MapAssets};

mod game;
mod resources;

fn main() {
    App::new()
        // Bevy Plugins
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "SCP-087-B Redux".into(),
                resolution: (1280., 720.).into(),
                ..default()
            }),
            ..default()
        }))
        // SCP-087-B Redux Plugins
        .add_plugins(GamePlugin)
        // Other Plugins
        .add_plugins((
            EntropyPlugin::<WyRand>::default(),
            bevy_panic_handler::PanicHandler::new().build(),
            PhysicsPlugins::default(),
            BillboardPlugin,
        ))
        .init_state::<GameState>()
        .add_loading_state(
            LoadingState::new(GameState::Preload)
                .continue_to_state(GameState::Game)
                .load_collection::<AudioAssets>()
                .load_collection::<MapAssets>(),
        )
        .run();
}

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
enum GameState {
    #[default]
    Preload,
    Game,
}
