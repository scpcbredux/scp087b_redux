use super::resources::{FloorAction, Map};
use crate::{
    game::{
        player::components::{Player, PlayerCamera},
        spawn_enemy,
    },
    resources::{AudioAssets, MapAssets},
};
use avian3d::prelude::*;
use bevy::prelude::*;
use bevy_rand::prelude::*;
use rand::prelude::*;

pub fn spawn_map(mut ambient_light: ResMut<AmbientLight>) {
    ambient_light.brightness = 80.0;
}

#[allow(clippy::too_many_arguments)]
pub fn update_floors(
    mut map: ResMut<Map>,
    mut commands: Commands,
    audio_assets: Res<AudioAssets>,
    mut p_query: Query<(&Player, &Transform, &mut LinearVelocity), Without<PlayerCamera>>,
    mut c_query: Query<&mut Transform, (With<PlayerCamera>, Without<Player>)>,
    mut rng: ResMut<GlobalEntropy<WyRand>>,
    mut ambient_light: ResMut<AmbientLight>,
    mut cur_enemy: Local<Option<Entity>>,
    map_assets: Res<MapAssets>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (player, p_transform, mut linear_velocity) in &mut p_query {
        if let Ok(mut c_transform) = c_query.get_single_mut() {
            let player_floor = player.floor_index;

            if map.floors[player_floor].timer > 0.0 {
                let floor_x = 4.0;
                let floor_y = -1.0 - (player_floor as f32 - 1.0) * 2.0;

                let (floor_z, start_x, end_x) =
                    if (player_floor as f32 / 2.0).floor() == (player_floor as f32 / 2.0).ceil() {
                        (6.5, 7.5, 0.5) // Even
                    } else {
                        (0.5, 0.5, 7.5) // Odd
                    };

                match map.floors[player_floor].action {
                    FloorAction::Proceed => {
                        map.floors[player_floor].timer += 1.0;
                        if map.floors[player_floor].timer == 150.0 {
                            commands.spawn(AudioBundle {
                                source: audio_assets.radio_sfx[0].clone(),
                                settings: PlaybackSettings::REMOVE,
                            });
                            map.floors[player_floor].timer = 0.0;
                        }
                    }
                    FloorAction::Radio2 => {
                        // signal seems to be getting weaker
                        commands.spawn(AudioBundle {
                            source: audio_assets.radio_sfx[1].clone(),
                            settings: PlaybackSettings::REMOVE,
                        });
                        map.floors[player_floor].timer = 0.0;
                    }
                    FloorAction::Radio3 => {
                        // good luck
                        commands.spawn(AudioBundle {
                            source: audio_assets.radio_sfx[2].clone(),
                            settings: PlaybackSettings::REMOVE,
                        });
                        map.floors[player_floor].timer = 0.0;
                    }
                    FloorAction::Radio4 => {
                        // M�RK�ILY�
                        commands.spawn(AudioBundle {
                            source: audio_assets.radio_sfx[3].clone(),
                            settings: PlaybackSettings::REMOVE,
                        });
                        map.floors[player_floor].timer = 0.0;
                    }
                    FloorAction::Flash => {
                        // m�rk� vilahtaa k�yt�v�n p��ss�
                        match map.floors[player_floor].timer {
                            1.0 => {
                                if p_transform
                                    .translation
                                    .distance(Vec3::new(end_x, floor_y, floor_z))
                                    < 1.5
                                {
                                    *cur_enemy = Some(spawn_enemy(
                                        &map_assets,
                                        &mut commands,
                                        &mut graphs,
                                        Vec3::new(end_x, floor_y - 0.5, floor_z),
                                        0.0,
                                    ));
                                    let horror_sfx =
                                        audio_assets.horror_sfx[rng.gen_range(0..2)].clone();
                                    commands.spawn(AudioBundle {
                                        source: horror_sfx,
                                        settings: PlaybackSettings::REMOVE,
                                    });
                                    map.floors[player_floor].timer = 5.0;
                                }
                            }
                            2.0 => {
                                if p_transform
                                    .translation
                                    .distance(Vec3::new(floor_x, floor_y, floor_z))
                                    < 1.5
                                {
                                    *cur_enemy = Some(spawn_enemy(
                                        &map_assets,
                                        &mut commands,
                                        &mut graphs,
                                        Vec3::new(floor_x, floor_y - 0.5, floor_z),
                                        0.0,
                                    ));
                                    let horror_sfx =
                                        audio_assets.horror_sfx[rng.gen_range(0..2)].clone();
                                    commands.spawn(AudioBundle {
                                        source: horror_sfx,
                                        settings: PlaybackSettings::REMOVE,
                                    });
                                    map.floors[player_floor].timer = 5.0;
                                }
                            }
                            3.0 => {
                                if p_transform
                                    .translation
                                    .distance(Vec3::new(start_x, floor_y, floor_z))
                                    < 1.5
                                {
                                    *cur_enemy = Some(spawn_enemy(
                                        &map_assets,
                                        &mut commands,
                                        &mut graphs,
                                        Vec3::new(start_x, floor_y - 0.5, floor_z),
                                        0.0,
                                    ));
                                    let horror_sfx =
                                        audio_assets.horror_sfx[rng.gen_range(0..2)].clone();
                                    commands.spawn(AudioBundle {
                                        source: horror_sfx,
                                        settings: PlaybackSettings::REMOVE,
                                    });
                                    map.floors[player_floor].timer = 5.0;
                                }
                            }
                            _ => {
                                map.floors[player_floor].timer += 1.0;
                                if map.floors[player_floor].timer > 30.0 {
                                    if let Some(entity) = *cur_enemy {
                                        commands.entity(entity).despawn_recursive();
                                    }

                                    map.floors[player_floor].timer = 0.0;
                                }
                            }
                        }
                    }
                    FloorAction::Lights => {
                        if map.floors[player_floor].timer == 1.0
                            && p_transform
                                .translation
                                .distance(Vec3::new(floor_x, floor_y, floor_z))
                                < 1.0
                        {
                            commands.spawn(AudioBundle {
                                source: audio_assets.horror_sfx[1].clone(),
                                settings: PlaybackSettings::REMOVE,
                            });
                            commands.spawn(AudioBundle {
                                source: audio_assets.fire_off.clone(),
                                settings: PlaybackSettings::REMOVE,
                            });
                            map.floors[player_floor].timer = 2.0;
                            ambient_light.brightness = 45.0;
                        }
                    }
                    FloorAction::Trick1 => {
                        if map.floors[player_floor].timer == 1.0 {
                            if (player_floor as f32 / 2.0).floor()
                                == (player_floor as f32 / 2.0).ceil()
                            {
                                // parillinen
                                if p_transform.translation.distance(Vec3::new(
                                    start_x - 1.5,
                                    floor_y - 0.5,
                                    floor_z - 5.0,
                                )) < 0.25
                                {
                                    // CurrEnemy = CreateEnemy(startx-1.5,FloorY-0.5,FloorZ-2.0,tex173)
                                    // CurrEnemy\speed = 0.01
                                    // EntityFX CurrEnemy\obj, 8
                                    map.floors[player_floor].timer = 2.0;
                                    commands.spawn(AudioBundle {
                                        source: audio_assets.horror_sfx[2].clone(),
                                        settings: PlaybackSettings::REMOVE,
                                    });
                                }
                            } else {
                                // pariton
                                if p_transform.translation.distance(Vec3::new(
                                    start_x + 1.5,
                                    floor_y - 0.5,
                                    floor_z + 5.0,
                                )) < 0.25
                                {
                                    // CurrEnemy = CreateEnemy(startx+1.5,FloorY-0.5,FloorZ+2.0,tex173)
                                    // CurrEnemy\speed = 0.01
                                    // EntityFX CurrEnemy\obj, 8
                                    map.floors[player_floor].timer = 2.0;
                                    commands.spawn(AudioBundle {
                                        source: audio_assets.horror_sfx[2].clone(),
                                        settings: PlaybackSettings::REMOVE,
                                    });
                                }
                            }
                        } /*else if distance2(transform, EntityX(CurrEnemy\collider), EntityY(CurrEnemy\collider), EntityZ(CurrEnemy\collider)) < 0.8 {
                              player.kill_timer = player.kill_timer.max(1.0);
                          }*/
                    }
                    FloorAction::Trick2 => {
                        if map.floors[player_floor].timer == 1.0 {
                            if (player_floor as f32 / 2.0).floor()
                                == (player_floor as f32 / 2.0).ceil()
                            {
                                // parillinen
                                if p_transform.translation.distance(Vec3::new(
                                    start_x + 0.5,
                                    floor_y - 0.5,
                                    floor_z - 5.0,
                                )) < 0.25
                                {
                                    // CurrEnemy = CreateEnemy(startx+0.5,FloorY-0.5,FloorZ-2.0,tex173)
                                    // CurrEnemy\speed = 0.01
                                    // EntityFX CurrEnemy\obj, 8
                                    map.floors[player_floor].timer = 2.0;
                                    commands.spawn(AudioBundle {
                                        source: audio_assets.horror_sfx[2].clone(),
                                        settings: PlaybackSettings::REMOVE,
                                    });
                                }
                            } else {
                                // pariton
                                if p_transform.translation.distance(Vec3::new(
                                    start_x - 0.5,
                                    floor_y - 0.5,
                                    floor_z + 5.0,
                                )) < 0.25
                                {
                                    // CurrEnemy = CreateEnemy(startx-0.5,FloorY-0.5,FloorZ+2.0,tex173)
                                    // CurrEnemy\speed = 0.01
                                    // EntityFX CurrEnemy\obj, 8
                                    map.floors[player_floor].timer = 2.0;
                                    commands.spawn(AudioBundle {
                                        source: audio_assets.horror_sfx[2].clone(),
                                        settings: PlaybackSettings::REMOVE,
                                    });
                                }
                            }
                        } /*else if distance2(transform, EntityX(CurrEnemy\collider), EntityY(CurrEnemy\collider), EntityZ(CurrEnemy\collider)) < 0.8 {
                              player.kill_timer = player.kill_timer.max(1.0);
                          }*/
                    }
                    FloorAction::Trap => {
                        if map.floors[player_floor].timer == 1.0 {
                            let translation = if (player_floor as f32 / 2.0).floor()
                                == (player_floor as f32 / 2.0).ceil()
                            {
                                Vec3::new(end_x + 0.5, floor_y, floor_z) // Even
                            } else {
                                Vec3::new(end_x - 0.5, floor_y, floor_z) // Odd
                            };
                            commands.spawn((
                                PbrBundle {
                                    mesh: meshes.add(Cuboid::new(1.0, 2.0, 1.0)),
                                    material: materials.add(StandardMaterial {
                                        base_color_texture: Some(
                                            map_assets.brick_wall_texture.clone(),
                                        ),
                                        ..default()
                                    }),
                                    transform: Transform {
                                        translation,
                                        rotation: Quat::from_rotation_y(f32::to_radians(-90.0)),
                                        ..default()
                                    },
                                    ..default()
                                },
                                Collider::cuboid(1.0, 2.0, 1.0),
                                RigidBody::Static,
                            ));
                            map.floors[player_floor].timer = 2.0;
                        } else if map.floors[player_floor].timer == 2.0
                            && p_transform
                                .translation
                                .distance(Vec3::new(floor_x, floor_y, floor_z))
                                < 1.0
                        {
                            *cur_enemy = Some(spawn_enemy(
                                &map_assets,
                                &mut commands,
                                &mut graphs,
                                Vec3::new(start_x, floor_y - 0.5, floor_z),
                                0.01,
                            ));
                            commands.spawn(AudioBundle {
                                source: audio_assets.horror_sfx[rng.gen_range(0..2)].clone(),
                                settings: PlaybackSettings::REMOVE,
                            });
                            map.floors[player_floor].timer = 3.0;
                        }
                    }
                    FloorAction::Roar => {
                        if map.floors[player_floor].timer == 1.0 {
                            if p_transform
                                .translation
                                .distance(Vec3::new(end_x, floor_y, floor_z))
                                < 6.0
                            {
                                // PositionEntity SoundEmitter,FloorX,FloorY-3,FloorZ
                                // TODO: Make 3d audio
                                commands.spawn(AudioBundle {
                                    source: audio_assets.roar_sfx.clone(),
                                    settings: PlaybackSettings::REMOVE,
                                });
                                map.floors[player_floor].timer = 51.0;
                            }
                        } else {
                            map.floors[player_floor].timer += 1.0;
                            if map.floors[player_floor].timer < 370.0 {
                                linear_velocity.0 += Vec3::new(
                                    rng.gen_range(-0.005..0.005),
                                    rng.gen_range(-0.005..0.005),
                                    rng.gen_range(-0.005..0.005),
                                );
                                c_transform.rotate(Quat::from_euler(
                                    EulerRot::XYZ,
                                    f32::to_radians(rng.gen_range(-1.0..1.0)),
                                    f32::to_radians(rng.gen_range(-1.0..1.0)),
                                    f32::to_radians(rng.gen_range(-1.0..1.0)),
                                ));
                            } else {
                                map.floors[player_floor].timer = 0.0;
                            }
                        }
                    }
                    FloorAction::Darkness => {
                        if map.floors[player_floor].timer == 1.0
                            && p_transform
                                .translation
                                .distance(Vec3::new(floor_x, floor_y, floor_z))
                                < 1.0
                        {
                            let translation = if (player_floor as f32 / 2.0).floor()
                                == (player_floor as f32 / 2.0).ceil()
                            {
                                Vec3::new(start_x - 0.5, floor_y, floor_z) // Even
                            } else {
                                Vec3::new(start_x + 0.5, floor_y, floor_z) // Odd
                            };
                            commands.spawn((
                                PbrBundle {
                                    mesh: meshes.add(Cuboid::new(1.0, 2.0, 1.0)),
                                    material: materials.add(StandardMaterial {
                                        base_color_texture: Some(
                                            map_assets.brick_wall_texture.clone(),
                                        ),
                                        ..default()
                                    }),
                                    transform: Transform {
                                        translation,
                                        rotation: Quat::from_rotation_y(f32::to_radians(-90.0)),
                                        ..default()
                                    },
                                    ..default()
                                },
                                Collider::cuboid(1.0, 2.0, 1.0),
                                RigidBody::Static,
                            ));

                            let translation = if (player_floor as f32 / 2.0).floor()
                                == (player_floor as f32 / 2.0).ceil()
                            {
                                Vec3::new(end_x + 0.5, floor_y, floor_z) // Even
                            } else {
                                Vec3::new(end_x - 0.5, floor_y, floor_z) // Odd
                            };
                            commands.spawn((
                                PbrBundle {
                                    mesh: meshes.add(Cuboid::new(1.0, 2.0, 1.0)),
                                    material: materials.add(StandardMaterial {
                                        base_color_texture: Some(
                                            map_assets.brick_wall_texture.clone(),
                                        ),
                                        ..default()
                                    }),
                                    transform: Transform {
                                        translation,
                                        rotation: Quat::from_rotation_y(f32::to_radians(-90.0)),
                                        ..default()
                                    },
                                    ..default()
                                },
                                Collider::cuboid(1.0, 2.0, 1.0),
                                RigidBody::Static,
                            ));

                            commands.spawn(AudioBundle {
                                source: audio_assets.stone_sfx.clone(),
                                settings: PlaybackSettings::REMOVE,
                            });
                            map.floors[player_floor].timer = 2.0;
                        } else if map.floors[player_floor].timer < 600.0 {
                            map.floors[player_floor].timer += 1.0;
                            // temp#=max(Brightness-(map.floors[player_floor].timer/600.0)*Brightness,10)
                            // AmbientLight temp,temp,temp

                            if map.floors[player_floor].timer == 600.0 {
                                *cur_enemy = Some(spawn_enemy(
                                    &map_assets,
                                    &mut commands,
                                    &mut graphs,
                                    Vec3::new(floor_x, floor_y - 0.5, floor_z),
                                    0.01,
                                ));
                                commands.spawn(AudioBundle {
                                    source: audio_assets.horror_sfx[rng.gen_range(0..2)].clone(),
                                    settings: PlaybackSettings::REMOVE,
                                });
                                map.floors[player_floor].timer = 601.0;
                            }
                        } /*else if Distance2(EntityX(CurrEnemy\collider),EntityY(CurrEnemy\collider),EntityZ(CurrEnemy\collider)) < 0.7 {
                              KillTimer = max(KillTimer,1)
                          }*/
                    }
                    _ => {}
                }
            }
        }
    }
}

pub fn floor_transform(i: usize) -> Transform {
    let mut transform = Transform::default();

    if (i as f32 / 2.0).floor() == (i as f32 / 2.0).ceil() {
        // parillinen
        transform.translation = Vec3::new(0.0, -(i as f32) * 2.0, 0.0);
    } else {
        // pariton
        transform.rotate_y(f32::to_radians(180.0));
        transform.translation = Vec3::new(8.0, -(i as f32) * 2.0, 7.0);
    }

    transform
}

pub fn room_label_transform(i: usize) -> Transform {
    let mut transform = Transform {
        rotation: Quat::from_rotation_x(f32::to_radians(-90.0)),
        ..default()
    };

    if (i as f32 / 2.0).floor() == (i as f32 / 2.0).ceil() {
        transform.translation = Vec3::new(-0.24, -(i as f32) * 2.0 - 0.6, 0.5);
        transform.rotate_y(f32::to_radians(180.0));
    } else {
        transform.translation = Vec3::new(7.4 + 0.6 + 0.24, -(i as f32) * 2.0 - 0.6, 6.0 + 0.5);
    }

    transform
}
