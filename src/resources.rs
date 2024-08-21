use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

#[derive(AssetCollection, Resource)]
#[allow(dead_code)]
pub struct AudioAssets {
    #[asset(path = "audio/step.wav")]
    pub step_sound: Handle<AudioSource>,
    #[asset(path = "audio/loudstep.ogg")]
    pub loud_step_sound: Handle<AudioSource>,
    #[asset(path = "audio/horror", collection(typed))]
    pub horror_sfx: Vec<Handle<AudioSource>>,
    #[asset(path = "audio/death.ogg")]
    pub death_sfx: Handle<AudioSource>,
    #[asset(path = "audio/roar.ogg")]
    pub roar_sfx: Handle<AudioSource>,
    #[asset(path = "audio/breath.ogg")]
    pub breath_sfx: Handle<AudioSource>,
    #[asset(path = "audio/stone.ogg")]
    pub stone_sfx: Handle<AudioSource>,
    #[asset(path = "audio/no.ogg")]
    pub no_sfx: Handle<AudioSource>,
    #[asset(path = "audio/ambient", collection(typed))]
    pub ambient_sfx: Vec<Handle<AudioSource>>,
    #[asset(path = "audio/dontlook.ogg")]
    pub dontlook_sfx: Handle<AudioSource>,
    #[asset(path = "audio/radio", collection(typed))]
    pub radio_sfx: Vec<Handle<AudioSource>>,
    #[asset(path = "audio/music.ogg")]
    pub music: Handle<AudioSource>,
    #[asset(path = "audio/match.ogg")]
    pub fire_on: Handle<AudioSource>,
    #[asset(path = "audio/fireout.ogg")]
    pub fire_off: Handle<AudioSource>,
}

#[derive(AssetCollection, Resource)]
pub struct MapAssets {
    #[asset(path = "map/map0.glb#Scene0")]
    pub map0: Handle<Scene>,
    #[asset(path = "map/map.glb#Scene0")]
    pub map: Handle<Scene>,
    #[asset(path = "map/map1.glb#Scene0")]
    pub map1: Handle<Scene>,
    #[asset(path = "map/map2.glb#Scene0")]
    pub map2: Handle<Scene>,
    #[asset(path = "map/map3.glb#Scene0")]
    pub map3: Handle<Scene>,
    #[asset(path = "map/map4.glb#Scene0")]
    pub map4: Handle<Scene>,
    #[asset(path = "map/map5.glb#Scene0")]
    pub map5: Handle<Scene>,
    #[asset(path = "map/map6.glb#Scene0")]
    pub map6: Handle<Scene>,
    #[asset(path = "map/maze.glb#Scene0")]
    pub map7: Handle<Scene>,
    #[asset(path = "map/door.jpg")]
    pub door_texture: Handle<Image>,
    #[asset(path = "map/Pretext.TTF")]
    pub font: Handle<Font>,
    #[asset(path = "map/sign.jpg")]
    pub sign_texture: Handle<Image>,
    #[asset(path = "map/glimpses", collection(typed))]
    pub glimpse_textures: Vec<Handle<Image>>,
}
