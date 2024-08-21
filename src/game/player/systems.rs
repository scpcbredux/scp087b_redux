use crate::{
    game::{
        components::{FloorLabel, FloorLabelUi},
        map_gen::{floor_transform, room_label_transform, Map},
        pooling::ObjectPool,
    },
    resources::{AudioAssets, MapAssets},
};

use super::{
    components::{Player, PlayerCamera},
    resources::{PlayerAction, PlayerInput},
    ANGLE_EPSILON,
};
use avian3d::prelude::*;
use bevy::prelude::*;
use bevy_rand::prelude::*;
use leafwing_input_manager::prelude::*;
use rand::prelude::*;
use std::{
    collections::HashSet,
    f32::consts::{FRAC_PI_2, PI, TAU},
    time::Duration,
};

pub fn player_input(
    query: Query<(&ActionState<PlayerAction>, &Player)>,
    windows: Query<&mut Window>,
    mut input: ResMut<PlayerInput>,
) {
    for (action_state, player) in &query {
        if let Ok(window) = windows.get_single() {
            if window.focused {
                let vector = action_state.axis_pair(&PlayerAction::MouseMotion);
                let mut delta = Vec2::ZERO;
                delta.x += vector.x;
                delta.y += vector.y;
                delta *= player.mouse_sensitivity;

                input.pitch = (input.pitch - delta.y)
                    .clamp(-FRAC_PI_2 + ANGLE_EPSILON, FRAC_PI_2 - ANGLE_EPSILON);
                input.yaw -= delta.x;
                if input.yaw.abs() > PI {
                    input.yaw = input.yaw.rem_euclid(TAU);
                }
            }
        }

        input.movement = Vec3::new(
            get_input_axis(
                &PlayerAction::MoveRight,
                &PlayerAction::MoveLeft,
                action_state,
            ),
            0.0,
            get_input_axis(&PlayerAction::MoveUp, &PlayerAction::MoveDown, action_state),
        )
        .normalize_or_zero();
    }
}

pub fn player_move(
    time: Res<Time>,
    mut query: Query<(&mut Player, &Transform, &mut LinearVelocity)>,
    input: Res<PlayerInput>,
    mut commands: Commands,
    audio_assets: Res<AudioAssets>,
) {
    let dt = time.delta_seconds();

    for (mut player, transform, mut linear_velocity) in &mut query {
        player.floor_index = ((-transform.translation.y - 0.5) as usize / 2) + 1;

        let mut move_to_world = Mat3::from_axis_angle(Vec3::Y, input.yaw);
        move_to_world.z_axis *= -1.0;

        let speed = 2.0;
        let y_component = linear_velocity.0.y;

        linear_velocity.0 = move_to_world * (input.movement * speed);
        linear_velocity.y = y_component;

        player
            .footstep_timer
            .tick(Duration::from_secs_f32(dt * linear_velocity.length()));

        if player.footstep_timer.finished() {
            commands.spawn(AudioBundle {
                source: audio_assets.step_sound.clone(),
                settings: PlaybackSettings::REMOVE,
            });
        }
    }
}

pub fn player_look(
    q_player: Query<(&Transform, &Player), Without<PlayerCamera>>,
    mut q_camera: Query<&mut Transform, (With<PlayerCamera>, Without<Player>)>,
    input: Res<PlayerInput>,
) {
    for (p_transform, player) in &q_player {
        for mut c_transform in &mut q_camera {
            c_transform.translation = p_transform.translation + player.camera_height;
            c_transform.rotation = Quat::from_euler(EulerRot::YXZ, input.yaw, input.pitch, 0.0);
        }
    }
}

pub fn player_cull_floor(
    map: Res<Map>,
    mut commands: Commands,
    map_assets: Res<MapAssets>,
    mut pool: ResMut<ObjectPool>,
    query: Query<&Player>,
) {
    if let Ok(player) = query.get_single() {
        let nearest_rooms = map.nearest_rooms_to_floor(player.floor_index, 1);

        // Track which rooms should remain active
        let mut new_active_rooms = HashSet::new();

        for &room_index in &nearest_rooms {
            if let Some(room_index) = room_index {
                pool.get_or_spawn(
                    room_index,
                    &map.rooms[room_index],
                    &mut commands,
                    &map_assets,
                    floor_transform(room_index),
                );
                new_active_rooms.insert(room_index);
            }
        }

        // Release rooms that are no longer near the player
        let current_active_rooms = pool.active_rooms.keys().cloned().collect::<Vec<_>>();
        for &room_index in &current_active_rooms {
            if !new_active_rooms.contains(&room_index) {
                let room_type = map.rooms[room_index].kind;
                pool.release(room_index, room_type);
            }
        }
    }
}

pub fn player_label_floor(
    map: Res<Map>,
    p_query: Query<&Player, (Without<FloorLabelUi>, Without<FloorLabel>)>,
    mut ui_query: Query<&mut Text, (With<FloorLabelUi>, Without<FloorLabel>, Without<Player>)>,
    mut l_query: Query<(&mut Transform, &mut Visibility), (With<FloorLabel>, Without<FloorLabelUi>, Without<Player>)>,
) {
    if let Ok(player) = p_query.get_single() {
        if let Some(label) = &map.rooms[player.floor_index - 1].label {
            if let Ok(mut text) = ui_query.get_single_mut() {
                text.sections[0].value = label.to_string();
            }

            for (mut transform, mut visibility) in &mut l_query {
                *transform = room_label_transform(player.floor_index - 1);
                *visibility = Visibility::Visible;
            }
        }
    }
}

pub fn player_death(
    mut query: Query<&mut Player>,
    mut commands: Commands,
    audio_assets: Res<AudioAssets>,
    mut rng: ResMut<GlobalEntropy<WyRand>>,
    // mut ambient_light: ResMut<AmbientLight>,
) {
    if let Ok(mut player) = query.get_single_mut() {
        if player.kill_timer > 0.0 {
            if player.kill_timer == 1.0 {
                commands.spawn(AudioBundle {
                    source: audio_assets.death_sfx.clone(),
                    settings: PlaybackSettings::ONCE,
                });
            }

            player.kill_timer += 1.0;

            // let secs = player.kill_timer.elapsed().as_secs_f32();
            // ambient_light.color = Color::srgb_from_array([255.0 - secs, 100.0 - secs, 100.0 - secs]);
            // RotateEntity camera, -KillTimer, EntityYaw(camera), EntityRoll(collider)-(KillTimer/2)

            if player.kill_timer > 90.0 && player.floor_index > 130 {
                match rng.gen_range(1..7) {
                    2 => panic!("It's not about whether you die or not, it's about when you die."),
                    3 => panic!("NICE"),
                    4 => panic!("welcome to NIL"),
                    _ => panic!("NO"),
                }
            }
        }
    }
}

fn get_input_axis<A: Actionlike>(paction: &A, saction: &A, action_state: &ActionState<A>) -> f32 {
    get_input_value(paction, action_state) - get_input_value(saction, action_state)
}

fn get_input_value<A: Actionlike>(action: &A, action_state: &ActionState<A>) -> f32 {
    if action_state.pressed(action) {
        1.0
    } else {
        0.0
    }
}
