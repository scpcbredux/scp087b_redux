use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use super::{components::*, resources::*};

#[derive(Bundle)]
pub struct PlayerBundle {
    pub player: Player,
    pub player_footsteps: PlayerFootsteps,
    pub input_bundle: InputManagerBundle<PlayerAction>,
}

impl Default for PlayerBundle {
    fn default() -> Self {
        Self {
            player: Default::default(),
            player_footsteps: Default::default(),
            input_bundle: InputManagerBundle::<PlayerAction> {
                action_state: ActionState::default(),
                input_map: InputMap::new([
                    (PlayerAction::MoveUp, KeyCode::KeyW),
                    (PlayerAction::MoveDown, KeyCode::KeyS),
                    (PlayerAction::MoveLeft, KeyCode::KeyA),
                    (PlayerAction::MoveRight, KeyCode::KeyD),
                ])
                .with_dual_axis(PlayerAction::MouseMotion, MouseMove::default()),
            },
        }
    }
}
