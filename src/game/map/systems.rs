use super::resources::{FloorAction, Map};
use crate::{game::player::components::Player, resources::AudioAssets};
use bevy::prelude::*;
use bevy_rand::prelude::*;
use rand::prelude::*;

pub fn spawn_map(mut ambient_light: ResMut<AmbientLight>) {
    ambient_light.brightness = 80.0;
}

pub fn update_floors(
    mut map: ResMut<Map>,
    mut commands: Commands,
    audio_assets: Res<AudioAssets>,
    query: Query<(&Player, &Transform)>,
    mut rng: ResMut<GlobalEntropy<WyRand>>,
    mut ambient_light: ResMut<AmbientLight>,
) {
    for (player, transform) in &query {
        let player_floor = player.floor_index;

        if map.floors[player_floor].timer > 0.0 {
            let floor_x = 4.0;
            let floor_y = -1.0 - (player_floor as f32 - 1.0) * 2.0;

            let (floor_z, start_x, end_x) =
                if (player_floor as f32 / 2.0).floor() == (player_floor as f32 / 2.0).ceil() {
                    // parillinen
                    (6.5, 7.5, 0.5)
                } else {
                    // pariton
                    (0.5, 0.5, 7.5)
                };

            match map.floors[player_floor].action {
                FloorAction::Proceed => {
                    map.floors[player_floor].timer += 1.0;
                    if map.floors[player_floor].timer == 150.0 {
                        commands.spawn(AudioBundle {
                            source: audio_assets.radio_sfx[0].clone(),
                            settings: PlaybackSettings::ONCE,
                        });
                        map.floors[player_floor].timer = 0.0;
                    }
                }
                FloorAction::Radio2 => {
                    // signal seems to be getting weaker
                    commands.spawn(AudioBundle {
                        source: audio_assets.radio_sfx[1].clone(),
                        settings: PlaybackSettings::ONCE,
                    });
                    map.floors[player_floor].timer = 0.0;
                }
                FloorAction::Radio3 => {
                    // good luck
                    commands.spawn(AudioBundle {
                        source: audio_assets.radio_sfx[2].clone(),
                        settings: PlaybackSettings::ONCE,
                    });
                    map.floors[player_floor].timer = 0.0;
                }
                FloorAction::Radio4 => {
                    // M�RK�ILY�
                    commands.spawn(AudioBundle {
                        source: audio_assets.radio_sfx[3].clone(),
                        settings: PlaybackSettings::ONCE,
                    });
                    map.floors[player_floor].timer = 0.0;
                }
                FloorAction::Flash => {
                    // m�rk� vilahtaa k�yt�v�n p��ss�
                    match map.floors[player_floor].timer {
                        1.0 => {
                            if transform
                                .translation
                                .distance(Vec3::new(end_x, floor_y, floor_z))
                                < 1.5
                            {
                                // CurrEnemy = CreateEnemy(EndX, FloorY-0.5, FloorZ,mental)
                                let horror_sfx =
                                    audio_assets.horror_sfx[rng.gen_range(0..2)].clone();
                                commands.spawn(AudioBundle {
                                    source: horror_sfx,
                                    settings: PlaybackSettings::ONCE,
                                });
                                map.floors[player_floor].timer = 5.0;
                            }
                        }
                        2.0 => {
                            if transform
                                .translation
                                .distance(Vec3::new(floor_x, floor_y, floor_z))
                                < 1.5
                            {
                                // CurrEnemy = CreateEnemy(FloorX, FloorY-0.5, FloorZ,mental)
                                let horror_sfx =
                                    audio_assets.horror_sfx[rng.gen_range(0..2)].clone();
                                commands.spawn(AudioBundle {
                                    source: horror_sfx,
                                    settings: PlaybackSettings::ONCE,
                                });
                                map.floors[player_floor].timer = 5.0;
                            }
                        }
                        3.0 => {
                            if transform
                                .translation
                                .distance(Vec3::new(start_x, floor_y, floor_z))
                                < 1.5
                            {
                                // CurrEnemy = CreateEnemy(startX, FloorY-0.5, FloorZ,mental)
                                let horror_sfx =
                                    audio_assets.horror_sfx[rng.gen_range(0..2)].clone();
                                commands.spawn(AudioBundle {
                                    source: horror_sfx,
                                    settings: PlaybackSettings::ONCE,
                                });
                                map.floors[player_floor].timer = 5.0;
                            }
                        }
                        _ => {
                            map.floors[player_floor].timer += 1.0;
                            if map.floors[player_floor].timer > 30.0 {
                                // FreeEntity CurrEnemy\collider
                                // FreeEntity CurrEnemy\obj
                                // Delete CurrEnemy

                                map.floors[player_floor].timer = 0.0;
                            }
                        }
                    }
                }
                FloorAction::Lights => {
                    if map.floors[player_floor].timer == 1.0
                        && transform
                            .translation
                            .distance(Vec3::new(floor_x, floor_y, floor_z))
                            < 1.0
                    {
                        commands.spawn(AudioBundle {
                            source: audio_assets.horror_sfx[1].clone(),
                            settings: PlaybackSettings::ONCE,
                        });
                        commands.spawn(AudioBundle {
                            source: audio_assets.fire_off.clone(),
                            settings: PlaybackSettings::ONCE,
                        });
                        map.floors[player_floor].timer = 2.0;
                        ambient_light.brightness = 45.0;
                    }
                }
                FloorAction::Trick1 => {
                    if map.floors[player_floor].timer == 1.0 {
                        if (player_floor as f32 / 2.0).floor() == (player_floor as f32 / 2.0).ceil()
                        {
                            // parillinen
                            if transform.translation.distance(Vec3::new(
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
                                    settings: PlaybackSettings::ONCE,
                                });
                            }
                        } else {
                            // pariton
                            if transform.translation.distance(Vec3::new(
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
                                    settings: PlaybackSettings::ONCE,
                                });
                            }
                        }
                    } /*else if distance2(transform, EntityX(CurrEnemy\collider), EntityY(CurrEnemy\collider), EntityZ(CurrEnemy\collider)) < 0.8 {
                          player.kill_timer = player.kill_timer.max(1.0);
                      }*/
                }
                FloorAction::Trick2 => {
                    if map.floors[player_floor].timer == 1.0 {
                        if (player_floor as f32 / 2.0).floor() == (player_floor as f32 / 2.0).ceil()
                        {
                            // parillinen
                            if transform.translation.distance(Vec3::new(
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
                                    settings: PlaybackSettings::ONCE,
                                });
                            }
                        } else {
                            // pariton
                            if transform.translation.distance(Vec3::new(
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
                                    settings: PlaybackSettings::ONCE,
                                });
                            }
                        }
                    } /*else if distance2(transform, EntityX(CurrEnemy\collider), EntityY(CurrEnemy\collider), EntityZ(CurrEnemy\collider)) < 0.8 {
                          player.kill_timer = player.kill_timer.max(1.0);
                      }*/
                }
                _ => {}
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
