use super::{
    components::{FloorLabel, FloorLabelUi, Glimpse},
    map_gen::{FloorAction, Map},
    player::{
        bundles::PlayerBundle,
        components::{Player, PlayerCamera},
    },
    FLOOR_AMOUNT,
};
use crate::resources::{AudioAssets, MapAssets};
use avian3d::prelude::*;
use bevy::{
    prelude::*,
    render::{
        camera::RenderTarget,
        render_resource::{
            Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
        },
    },
    window::CursorGrabMode,
};
use bevy_mod_billboard::prelude::*;
use bevy_rand::prelude::*;
use rand::prelude::*;

pub fn create_player(
    mut commands: Commands,
    audio_assets: Res<AudioAssets>,
    mut windows: Query<&mut Window>,
) {
    let mut window = windows.single_mut();
    window.cursor.visible = false;
    window.cursor.grab_mode = CursorGrabMode::Locked;

    // Music
    commands.spawn(AudioBundle {
        source: audio_assets.music.clone(),
        settings: PlaybackSettings::LOOP,
    });

    // Player
    commands.spawn((
        Name::new("Player"),
        Collider::capsule(0.3, 1.0),
        Friction::ZERO.with_combine_rule(CoefficientCombine::Min),
        Restitution::ZERO.with_combine_rule(CoefficientCombine::Min),
        RigidBody::Dynamic,
        LockedAxes::ROTATION_LOCKED,
        GravityScale(4.0),
        Position::from_xyz(-1.5, -1.0, 0.5),
        TransformBundle::default(),
        PlayerBundle::default(),
    ));

    // Player Camera
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(-1.5, -1.0, 0.5),
            projection: Projection::Perspective(PerspectiveProjection {
                far: 1.0,
                ..default()
            }),
            ..default()
        },
        FogSettings {
            color: Color::srgb(0.0, 0.0, 0.0),
            falloff: FogFalloff::Linear {
                start: 1.0,
                end: 2.5,
            },
            ..default()
        },
        PlayerCamera::default(),
        SpatialListener::new(4.0),
    ));
}

pub fn create_map(
    mut commands: Commands,
    map_assets: Res<MapAssets>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut rng: ResMut<GlobalEntropy<WyRand>>,
    mut ambient_light: ResMut<AmbientLight>,
    mut images: ResMut<Assets<Image>>,
) {
    ambient_light.brightness = 40.0;

    // Door
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Cuboid::new(1.0, 2.0, 1.0)),
            material: materials.add(StandardMaterial {
                base_color_texture: Some(map_assets.door_texture.clone()),
                ..default()
            }),
            transform: Transform {
                translation: Vec3::new(-3.5, -1.0, 0.5),
                rotation: Quat::from_rotation_y(f32::to_radians(-90.0)),
                ..default()
            },
            ..default()
        },
        Collider::cuboid(1.0, 2.0, 1.0),
        RigidBody::Static,
    ));

    // Generate Map
    let mut map = Map::new(FLOOR_AMOUNT);
    map.generate(&mut rng);
    commands.insert_resource(map);

    // Floor Label
    let size = Extent3d {
        width: 512,
        height: 512,
        ..default()
    };

    let mut image = Image {
        texture_descriptor: TextureDescriptor {
            label: None,
            size,
            dimension: TextureDimension::D2,
            format: TextureFormat::Bgra8UnormSrgb,
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        },
        ..default()
    };

    // fill image.data with zeroes
    image.resize(size);

    let image_handle = images.add(image);

    let texture_camera = commands
        .spawn(Camera2dBundle {
            camera: Camera {
                // render before the "main pass" camera
                order: -1,
                target: RenderTarget::Image(image_handle.clone()),
                ..default()
            },
            ..default()
        })
        .id();

    commands
        .spawn((
            ImageBundle {
                style: Style {
                    // Cover the whole image
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                image: map_assets.sign_texture.clone().into(),
                ..default()
            },
            TargetCamera(texture_camera),
        ))
        .with_children(|parent| {
            parent.spawn((
                TextBundle::from_section(
                    "2",
                    TextStyle {
                        font: map_assets.font.clone(),
                        font_size: 156.0,
                        color: Color::BLACK,
                    },
                ),
                FloorLabelUi,
            ));
        });

    // Map0
    commands.spawn((
        SceneBundle {
            scene: map_assets.map0.clone(),
            ..default()
        },
        ColliderConstructorHierarchy::new(Some(ColliderConstructor::TrimeshFromMesh)),
        RigidBody::Static,
    ));

    commands.spawn((
        PbrBundle {
            visibility: Visibility::Hidden,
            mesh: meshes.add(Cuboid::new(0.5, 0.5, 0.5)),
            material: materials.add(StandardMaterial {
                base_color_texture: Some(image_handle.clone()),
                ..default()
            }),
            ..default()
        },
        FloorLabel,
    ));
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
                        ambient_light.brightness = 25.0;
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

#[allow(clippy::type_complexity)]
pub fn update_glimpses(
    mut commands: Commands,
    audio_assets: Res<AudioAssets>,
    g_query: Query<(&Transform, Entity), (With<Glimpse>, Without<Player>)>,
    p_query: Query<(&Player, &Transform), Without<Glimpse>>,
) {
    for (player, p_transform) in &p_query {
        for (g_transform, g_entity) in &g_query {
            if player.floor_index - 1 == ((-g_transform.translation.y - 0.5) / 2.0) as usize
                && p_transform.translation.distance(Vec3::new(
                    g_transform.translation.x,
                    g_transform.translation.y,
                    g_transform.translation.z,
                )) < 2.3
            {
                // TODO: Make a 3d audio
                commands.spawn(AudioBundle {
                    source: audio_assets.no_sfx.clone(),
                    settings: PlaybackSettings::DESPAWN,
                });

                commands.entity(g_entity).despawn();
            }
        }
    }
}

pub fn create_glimpses(
    mut commands: Commands,
    map: Res<Map>,
    map_assets: Res<MapAssets>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut rng: ResMut<GlobalEntropy<WyRand>>,
) {
    let glimpse_mesh = meshes.add(Rectangle::new(0.6, 0.6));

    // TODO: Maybe include this in the pooling?
    for (i, floor) in map.floors.iter().enumerate() {
        if floor.action != FloorAction::Steps || rng.gen_range(1..7) != 1 {
            continue;
        }

        let floor_y = -((i as f32 - 1.0) * 2.0 + 1.0);
        let start_x = 0.8;
        let end_x = 7.2;
        let floor_z = if i % 2 == 0 {
            6.55 // Even index
        } else {
            0.3 // Odd index
        };

        let floor_x = rng.gen_range(start_x..end_x);

        let glimpse_texture = map_assets.glimpse_textures[rng.gen_range(0..1)].clone();

        commands.spawn((
            BillboardTextureBundle {
                transform: Transform::from_xyz(floor_x, floor_y, floor_z),
                texture: BillboardTextureHandle(glimpse_texture.clone()),
                mesh: BillboardMeshHandle(glimpse_mesh.clone()),
                ..default()
            },
            Glimpse,
        ));
    }
}
