use super::{
    enemy::components::{Enemy, EnemyAnimations},
    glimpse::components::Glimpse,
    map::{
        components::{FloorLabel, FloorLabelUi},
        resources::{FloorAction, Map},
    },
    player::{bundles::PlayerBundle, components::PlayerCamera},
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
};
use bevy_mod_billboard::prelude::*;
use bevy_rand::prelude::*;
use rand::prelude::*;

pub fn spawn_map(
    map_assets: Res<MapAssets>,
    audio_assets: Res<AudioAssets>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut rng: ResMut<GlobalEntropy<WyRand>>,
) {
    // Music
    commands.spawn(AudioBundle {
        source: audio_assets.music.clone(),
        settings: PlaybackSettings::LOOP,
    });

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
    let mut map = Map::default();
    map.generate(&mut rng);

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

    commands.insert_resource(map);
}

pub fn spawn_player(mut commands: Commands) {
    // Player
    commands.spawn((
        Name::new("Player"),
        Collider::capsule(0.3, 1.0),
        Friction::ZERO.with_combine_rule(CoefficientCombine::Min),
        Restitution::ZERO.with_combine_rule(CoefficientCombine::Min),
        RigidBody::Dynamic,
        LockedAxes::ROTATION_LOCKED,
        GravityScale(1.0),
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

pub fn spawn_glimpses(
    map: Res<Map>,
    map_assets: Res<MapAssets>,
    mut commands: Commands,
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

pub fn spawn_enemy(
    map_assets: &Res<MapAssets>,
    commands: &mut Commands,
    graphs: &mut ResMut<Assets<AnimationGraph>>,
    position: Vec3,
    speed: f32,
) -> Entity {
    // Build the animation graph
    let mut graph = AnimationGraph::new();
    let animations = graph
        .add_clips(map_assets.mental_animations.clone(), 1.0, graph.root)
        .collect();

    // Insert a resource with the current scene information
    let graph = graphs.add(graph);

    // Enemy
    commands
        .spawn((
            Name::new("Enemy"),
            SpatialBundle::default(),
            Collider::capsule(0.3, 1.0),
            Friction::ZERO.with_combine_rule(CoefficientCombine::Min),
            Restitution::ZERO.with_combine_rule(CoefficientCombine::Min),
            RigidBody::Dynamic,
            LockedAxes::new().lock_rotation_x().lock_rotation_z(),
            GravityScale(1.0),
            Position::new(position),
            Enemy { speed },
            EnemyAnimations {
                animations,
                graph: graph.clone(),
            },
        ))
        .with_children(|parent| {
            parent.spawn(SceneBundle {
                scene: map_assets.mental_model.clone(),
                transform: Transform {
                    translation: Vec3::Y * -0.7,
                    rotation: Quat::from_rotation_y(f32::to_radians(180.0)),
                    scale: Vec3::splat(0.17),
                },
                ..default()
            });
        })
        .id()
}
