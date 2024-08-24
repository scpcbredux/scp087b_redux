use crate::AppState;
use bevy::prelude::*;
use systems::update_glimpses;

pub mod components;
mod systems;

pub struct GlimpsePlugin;

impl Plugin for GlimpsePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_glimpses.run_if(in_state(AppState::Game)));
    }
}
