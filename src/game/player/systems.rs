use crate::{
    game::{
        components::{FloorLabel, FloorLabelUi},
        map_gen::{floor_transform, room_label_transform, Map},
        pooling::ObjectPool,
    },
    resources::{AudioAssets, MapAssets},
};

use super::{components::*, resources::*, ANGLE_EPSILON};
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
    mut query: Query<(&mut Player, &Transform, &mut LinearVelocity)>,
    input: Res<PlayerInput>,
) {
    for (mut player, transform, mut linear_velocity) in &mut query {
        player.floor_index = ((-transform.translation.y - 0.5) as usize / 2) + 1;

        let mut move_to_world = Mat3::from_axis_angle(Vec3::Y, input.yaw);
        move_to_world.z_axis *= -1.0;

        let speed = 2.0;
        let y_component = linear_velocity.0.y;

        linear_velocity.0 = move_to_world * (input.movement * speed);
        linear_velocity.y = y_component;
    }
}

pub fn player_look(
    time: Res<Time>,
    q_player: Query<(&Transform, &LinearVelocity, &Player), Without<PlayerCamera>>,
    mut q_camera: Query<(&mut PlayerCamera, &mut Transform), Without<Player>>,
    input: Res<PlayerInput>,
) {
    let dt = time.delta_seconds();

    for (p_transform, linear_velocity, player) in &q_player {
        for (mut camera, mut c_transform) in &mut q_camera {
            camera.timer += dt * linear_velocity.length() / player.speed;

            let c_off = Vec3::new(
                (camera.timer * camera.speed / 2.0).cos(),
                -(camera.timer * camera.speed).sin(),
                0.0,
            );

            let rot = -(camera.timer * camera.speed / 2.0).cos() * camera.tilt;

            c_transform.translation =
                p_transform.translation + player.camera_height + c_off * camera.max_bob;
            c_transform.rotation = Quat::from_euler(EulerRot::YXZ, input.yaw, input.pitch, 0.0)
                * Quat::from_rotation_z(rot);
        }
    }
}

pub fn player_footsteps(
    time: Res<Time>,
    mut commands: Commands,
    audio_assets: Res<AudioAssets>,
    mut query: Query<(Entity, &LinearVelocity, &mut PlayerFootsteps, &Transform), With<Player>>,
) {
    let dt = time.delta_seconds();

    for (entity, linear_velocity, mut footsteps, _transform) in &mut query {
        footsteps
            .timer
            .tick(Duration::from_secs_f32(dt * linear_velocity.length()));

        if footsteps.timer.finished() {
            commands.entity(entity).insert(AudioBundle {
                source: audio_assets.step_sound.clone(),
                settings: PlaybackSettings::REMOVE,
            });
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

#[allow(clippy::type_complexity)]
pub fn player_label_floor(
    map: Res<Map>,
    p_query: Query<&Player, (Without<FloorLabelUi>, Without<FloorLabel>)>,
    mut ui_query: Query<&mut Text, (With<FloorLabelUi>, Without<FloorLabel>, Without<Player>)>,
    mut l_query: Query<
        (&mut Transform, &mut Visibility),
        (With<FloorLabel>, Without<FloorLabelUi>, Without<Player>),
    >,
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
    time: Res<Time>,
    mut query: Query<&mut Player>,
    mut commands: Commands,
    audio_assets: Res<AudioAssets>,
    mut rng: ResMut<GlobalEntropy<WyRand>>,
    // mut ambient_light: ResMut<AmbientLight>,
) {
    let dt = time.delta_seconds();

    if let Ok(mut player) = query.get_single_mut() {
        if player.kill_timer > 0.0 {
            if player.kill_timer == 1.0 {
                commands.spawn(AudioBundle {
                    source: audio_assets.death_sfx.clone(),
                    settings: PlaybackSettings::ONCE,
                });
            }

            player.kill_timer += 1.0 * dt;

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

pub fn player_ambience(
    mut commands: Commands,
    audio_assets: Res<AudioAssets>,
    mut rng: ResMut<GlobalEntropy<WyRand>>,
) {
    if rng.gen_range(1..1000) < 2 {
        // TODO: Make 3d audio.
        commands.spawn(AudioBundle {
            source: audio_assets.ambient_sfx[rng.gen_range(0..8)].clone(),
            settings: PlaybackSettings::ONCE,
        });
    }
}

pub fn player_fall_damage(mut query: Query<(&mut Player, &LinearVelocity)>) {
    // for (mut player, linear_velocity) in &mut query {
    //     info!("Player Y Velocity: {:#?}", linear_velocity.y);

    //     if linear_velocity.y < -0.09 {
    //         player.kill_timer = player.kill_timer.max(1.0);
    //     }
    // }
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
