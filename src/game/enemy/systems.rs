use super::{components::Enemy, resources::Animations};
use crate::game::player::components::Player;
use avian3d::prelude::*;
use bevy::prelude::*;
use std::time::Duration;

pub fn enemies_once_loaded(
    mut commands: Commands,
    animations: Res<Animations>,
    mut players: Query<(Entity, &mut AnimationPlayer), Added<AnimationPlayer>>,
) {
    for (entity, mut player) in &mut players {
        let mut transitions = AnimationTransitions::new();

        // Make sure to start the animation via the `AnimationTransitions`
        // component. The `AnimationTransitions` component wants to manage all
        // the animations and will get confused if the animations are started
        // directly via the `AnimationPlayer`.
        transitions
            .play(&mut player, animations.animations[0], Duration::ZERO)
            .repeat();

        commands
            .entity(entity)
            .insert(animations.graph.clone())
            .insert(transitions);
    }
}

pub fn enemies_update(
    mut e_query: Query<(&mut Transform, &mut LinearVelocity, &Visibility, &Enemy), Without<Player>>,
    p_query: Query<&Transform, (With<Player>, Without<Enemy>)>,
) {
    if let Ok(p_transform) = p_query.get_single() {
        for (mut e_transform, mut linear_velocity, visibility, enemy) in &mut e_query {
            if *visibility != Visibility::Hidden {
                e_transform.look_at(p_transform.translation, Vec3::Y);
                let direction = p_transform.translation - e_transform.translation;
                if direction.length() > 1.5 {
                    let movement_direction = direction.normalize();
                    linear_velocity.0 += movement_direction * enemy.speed;
                }
                // BlurTimer = 200;
            }
        }
    }
}

pub fn enemies_animation(
    mut animation_players: Query<
        (
            &mut AnimationPlayer,
            &mut AnimationTransitions,
            &LinearVelocity,
        ),
        With<Enemy>,
    >,
    animations: Res<Animations>,
) {
    for (mut player, mut transitions, linear_velocity) in &mut animation_players {
        let current_animation = if linear_velocity.length() > 0.0 {
            animations.animations[1]
        } else {
            animations.animations[0]
        };

        if !player.is_playing_animation(current_animation) {
            transitions
                .play(&mut player, current_animation, Duration::ZERO)
                .repeat();
        }
    }
}
