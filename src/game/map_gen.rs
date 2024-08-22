use bevy::prelude::*;
use bevy_rand::prelude::*;
use rand::Rng;

#[derive(PartialEq, Eq, Default, Clone, Copy, Debug)]
pub enum FloorAction {
    #[default]
    Steps,
    Lights,
    Flash,
    Run,
    Breath,
    Proceed,
    Trap,
    Scp173,
    Cell,
    Lock,
    Radio2,
    Radio3,
    Radio4,
    Trick1,
    Trick2,
    Roar,
    Darkness,
}

#[derive(Default, Clone, Debug)]
pub struct Floor {
    pub action: FloorAction,
    pub timer: f32,
}

#[derive(PartialEq, Eq, Default, Clone, Copy, Debug, Hash)]
pub enum RoomType {
    Map,
    #[default]
    Map0,
    Map1,
    Map2,
    Map3,
    Map4,
    Map5,
    Map6,
    Maze,
}

#[derive(Default, Clone, Debug)]
pub struct Room {
    pub kind: RoomType,
    pub label: Option<String>,
}

#[derive(Resource, Clone, Debug)]
pub struct Map {
    pub floor_amount: usize,
    pub floors: Vec<Floor>,
    pub rooms: Vec<Room>,
}

impl Map {
    pub fn new(floor_amount: usize) -> Self {
        Self {
            floor_amount,
            floors: vec![Floor::default(); floor_amount],
            rooms: Default::default(),
        }
    }

    pub fn generate(&mut self, rng: &mut ResMut<GlobalEntropy<WyRand>>) {
        self.assign_floor_action(1, FloorAction::Proceed, 1.0);

        if rng.gen_bool(0.5) {
            self.assign_random_floor_action(3..4, FloorAction::Radio2, 1.0);
        }

        if rng.gen_bool(2.0 / 3.0) {
            self.assign_random_floor_action(5..6, FloorAction::Radio3, 1.0);
        }

        self.assign_floor_action(7, FloorAction::Lock, 1.0);

        if rng.gen_bool(0.5) {
            self.assign_random_floor_action(8..9, FloorAction::Radio4, 1.0);
        }

        self.assign_random_floor_action(10..11, FloorAction::Breath, 1.0);
        self.assign_random_floor_action(12..13, FloorAction::Steps, 1.0);
        self.assign_random_floor_action(10..19, FloorAction::Flash, 1.0);
        self.assign_random_floor_action(20..22, FloorAction::Lights, 1.0);

        match rng.gen_range(0..4) {
            1 => self.assign_random_floor_action(25..28, FloorAction::Trick1, 1.0),
            2 => self.assign_random_floor_action(25..28, FloorAction::Trick2, 1.0),
            _ => {}
        }

        self.assign_random_floor_action(29..33, FloorAction::Run, 1.0);
        self.assign_random_floor_action(34..37, FloorAction::Scp173, 1.0);

        for _ in 0..8 {
            let rand_action = match rng.gen_range(1..10) {
                2 => FloorAction::Flash,
                3 => FloorAction::Trick1,
                4 => FloorAction::Trick2,
                5 => FloorAction::Breath,
                6 => FloorAction::Steps,
                7 => FloorAction::Trap,
                8 => FloorAction::Roar,
                _ => FloorAction::Cell,
            };

            loop {
                let temp = rng.gen_range(25..69);
                if self.floors[temp].action == FloorAction::Steps {
                    self.floors[temp].action = rand_action;
                    break;
                }
            }
        }

        for _ in 0..60 {
            let rand_action = match rng.gen_range(1..10) {
                2 => FloorAction::Lights,
                3 => FloorAction::Run,
                4 => FloorAction::Trick2,
                5 => FloorAction::Breath,
                6 => FloorAction::Steps,
                7 => FloorAction::Trap,
                8 => FloorAction::Roar,
                _ => FloorAction::Cell,
            };

            loop {
                let temp = rng.gen_range(75..200);
                if self.floors[temp].action == FloorAction::Steps {
                    self.floors[temp].action = rand_action;
                    break;
                }
            }
        }

        self.assign_random_floor_action(150..200, FloorAction::Darkness, 1.0);
        self.gen_rooms(rng);
    }

    fn assign_floor_action(&mut self, index: usize, action: FloorAction, duration: f32) {
        if let Some(floor) = self.floors.get_mut(index) {
            floor.action = action;
            floor.timer = duration;
        }
    }

    fn assign_random_floor_action(
        &mut self,
        range: std::ops::Range<usize>,
        action: FloorAction,
        duration: f32,
    ) {
        let temp = rand::thread_rng().gen_range(range);
        self.assign_floor_action(temp, action, duration);
    }

    fn gen_rooms(&mut self, rng: &mut ResMut<GlobalEntropy<WyRand>>) {
        for i in 0..self.floor_amount - 1 {
            let kind = if i == 0 {
                RoomType::Map0
            } else {
                match self.floors[i + 1].action {
                    FloorAction::Scp173 => RoomType::Map2,
                    FloorAction::Cell => RoomType::Map1,
                    FloorAction::Trick1 => RoomType::Map4,
                    FloorAction::Trick2 => RoomType::Map5,
                    FloorAction::Flash
                    | FloorAction::Run
                    | FloorAction::Lights
                    | FloorAction::Trap
                    | FloorAction::Lock => RoomType::Map,
                    FloorAction::Steps => match rng.gen_range(0..20) {
                        1 | 2 => RoomType::Map1,
                        3 | 4 => RoomType::Map2,
                        5 | 6 => RoomType::Map3,
                        7 => RoomType::Map4,
                        8 => RoomType::Map5,
                        9 => RoomType::Map6,
                        10 => {
                            if i > 40 {
                                RoomType::Maze
                            } else {
                                RoomType::Map
                            }
                        }
                        _ => RoomType::Map,
                    },
                    _ => RoomType::Map,
                }
            };

            let label = if i == 0 {
                None
            } else {
                let mut label = match rng.gen_range(0..600) {
                    1 => String::new(),
                    2 => rng.gen_range(33..122).to_string(),
                    3 => "NIL".to_string(),
                    4 => "?".to_string(),
                    5 => "NO".to_string(),
                    6 => "stop".to_string(),
                    _ => (i + 1).to_string(),
                };

                if i > 140 {
                    label = String::new();
                    for _ in 1..rng.gen_range(1..4) {
                        label += &rng.gen_range(33..122).to_string();
                    }
                }

                Some(label)
            };

            self.rooms.push(Room { kind, label });
        }
    }

    /// Grabs the nearest floors based on the current floor and options
    ///
    /// [0] = above room (if available)
    /// [1] = current room
    /// [2] = bottom room (if available)
    pub fn nearest_rooms_to_floor(&self, cur_floor: usize, distance: usize) -> [Option<usize>; 3] {
        let above_room = if cur_floor >= distance {
            Some(cur_floor - distance)
        } else {
            None
        };

        let current_room = Some(cur_floor);

        let bottom_room = if cur_floor + distance < self.floor_amount {
            Some(cur_floor + distance)
        } else {
            None
        };

        [above_room, current_room, bottom_room]
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
