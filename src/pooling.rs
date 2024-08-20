use crate::{map_gen::RoomType, MapAssets};
use avian3d::prelude::*;
use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Resource, Default)]
pub struct ObjectPool {
    available_rooms: HashMap<RoomType, Vec<Entity>>,
    pub active_rooms: HashMap<usize, Entity>, // Maps room indices to active entities
}

impl ObjectPool {
    pub fn get_or_spawn(
        &mut self,
        room_index: usize,
        room_type: RoomType,
        commands: &mut Commands,
        map_assets: &Res<MapAssets>,
        transform: Transform,
    ) -> Entity {
        // Check if the room is already active
        if let Some(&entity) = self.active_rooms.get(&room_index) {
            // Update the transform if the room is already active
            commands.entity(entity).insert(transform);
            return entity;
        }

        // Get the available rooms list, or create an empty list if none exists
        let available_rooms = self.available_rooms.entry(room_type).or_default();

        let entity = if let Some(entity) = available_rooms.pop() {
            commands.entity(entity).insert(transform);
            entity
        } else {
            // Spawn a new entity if none are available
            let scene = match room_type {
                RoomType::Map => map_assets.map.clone(),
                RoomType::Map0 => map_assets.map0.clone(),
                RoomType::Map1 => map_assets.map1.clone(),
                RoomType::Map2 => map_assets.map2.clone(),
                RoomType::Map3 => map_assets.map3.clone(),
                RoomType::Map4 => map_assets.map4.clone(),
                RoomType::Map5 => map_assets.map5.clone(),
                RoomType::Map6 => map_assets.map6.clone(),
                RoomType::Maze => map_assets.map7.clone(),
            };

            let new_entity = commands
                .spawn((
                    SceneBundle {
                        scene,
                        transform,
                        ..default()
                    },
                    ColliderConstructorHierarchy::new(Some(ColliderConstructor::TrimeshFromMesh)),
                    RigidBody::Static,
                ))
                .id();
            new_entity
        };

        // Mark this room as active
        self.active_rooms.insert(room_index, entity);
        entity
    }

    pub fn release(&mut self, room_index: usize, room_type: RoomType) {
        if let Some(entity) = self.active_rooms.remove(&room_index) {
            self.available_rooms
                .entry(room_type)
                .or_default()
                .push(entity);
        }
    }
}
