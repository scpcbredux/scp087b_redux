use super::components::Glimpse;
use crate::{game::player::components::Player, resources::AudioAssets};
use bevy::prelude::*;

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
