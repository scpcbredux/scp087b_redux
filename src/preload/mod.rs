use crate::{resources::MapAssets, AppState};
use bevy::prelude::*;

#[derive(Component)]
pub struct PreloadComponent;

pub struct PreloadPlugin;

impl Plugin for PreloadPlugin {
    fn build(&self, app: &mut App) {
        app
            // OnEnter State Systems
            .add_systems(OnEnter(AppState::Preload), spawn_preload)
            // OnExit State Systems
            .add_systems(OnExit(AppState::Preload), despawn_preload)
            // Systems
            .add_systems(Update, update_preload.run_if(in_state(AppState::Preload)));
    }
}

fn spawn_preload(mut commands: Commands, map_assets: Res<MapAssets>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(30.0),
                    ..default()
                },
                background_color: Color::BLACK.into(),
                ..default()
            },
            PreloadComponent,
        ))
        .with_children(|parent| {
            parent.spawn(ImageBundle {
                style: Style {
                    width: Val::Percent(40.0),
                    height: Val::Percent(32.0),
                    ..default()
                },
                image: map_assets.scp_logo.clone().into(),
                ..default()
            });
            parent.spawn(TextBundle::from_section(
                "Press Space to Continue",
                TextStyle {
                    font_size: 35.0,
                    color: Color::WHITE,
                    font: map_assets.font.clone(),
                },
            ));
        });

    commands.spawn((Camera2dBundle::default(), PreloadComponent));
}

fn despawn_preload(mut commands: Commands, query: Query<Entity, With<PreloadComponent>>) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}

fn update_preload(
    keys: Res<ButtonInput<KeyCode>>,
    mut next_app_state: ResMut<NextState<AppState>>,
) {
    if keys.just_pressed(KeyCode::Space) {
        next_app_state.set(AppState::Game);
    }
}
