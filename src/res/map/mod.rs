mod rect;
use bevy::prelude::Commands;
use bracket_bevy::RandomNumbers;
use rect::Rect;

#[derive(PartialEq, Copy, Clone)]
pub enum TileType {
    Wall,
    Floor,
}
#[derive(Clone)]
pub struct Map {
    pub tiles: Vec<TileType>,
    pub rooms: Vec<Rect>,
    pub width: i32,
    pub height: i32,
    pub revealed_tiles: Vec<bool>,
    pub visible_tiles: Vec<bool>,
}

impl Default for Map {
    fn default() -> Self {
        Self {
            tiles: vec![TileType::Wall; 80 * 50],
            rooms: Default::default(),
            width: 80,
            height: 50,
            revealed_tiles: vec![false; 80 * 50],
            visible_tiles: vec![false; 80 * 50]
        }
    }
}

pub fn xy_idx(x: i32, y: i32) -> usize {
    (y as usize * 80) + x as usize
}

struct MapBuilder {
    tiles: Vec<TileType>,
    rooms: Vec<Rect>,
}

impl Default for MapBuilder {
    fn default() -> Self {
        Self {
            tiles: vec![TileType::Wall; 80 * 50],
            rooms: Default::default(),
        }
    }
}
impl MapBuilder {
    fn build(self) -> Map {
        Map {
            tiles: self.tiles,
            rooms: self.rooms,
            ..Default::default()
        }
    }
    fn apply_room_to_map(&mut self, room: &Rect) {
        for y in room.y.0 + 1..=room.y.1 {
            for x in room.x.0 + 1..=room.x.1 {
                self.tiles[xy_idx(x, y)] = TileType::Floor;
            }
        }
    }
    fn apply_horizontal_tunnel(&mut self, x: (i32, i32), y: i32) {
        for x in x.0.min(x.1)..=x.0.max(x.1) {
            let idx = xy_idx(x, y);
            if idx > 0 && idx < self.tiles.len() {
                self.tiles[idx as usize] = TileType::Floor;
            }
        }
    }
    fn apply_vertical_tunnel(&mut self, y: (i32, i32), x: i32) {
        for y in y.0.min(y.1)..=y.0.max(y.1) {
            let idx = xy_idx(x, y);
            if idx > 0 && idx < self.tiles.len() {
                self.tiles[idx as usize] = TileType::Floor;
            }
        }
    }
}

pub fn setup(commands: &mut Commands) -> Map {
    const MAX_ROOMS: i32 = 30;
    const MIN_SIZE: i32 = 6;
    const MAX_SIZE: i32 = 10;
    let mut map_builder = MapBuilder::default();
    let rng = RandomNumbers::default();
    for _ in 0..MAX_ROOMS {
        let w = rng.range(MIN_SIZE, MAX_SIZE);
        let h = rng.range(MIN_SIZE, MAX_SIZE);
        let x = rng.roll_dice(1, 80 - w - 1) - 1;
        let y = rng.roll_dice(1, 50 - h - 1) - 1;
        let new_room = Rect::new(x, y, w, h);
        let mut ok = true;
        for other_room in map_builder.rooms.iter() {
            if new_room.intersect(other_room) {
                ok = false
            }
        }
        if ok {
            map_builder.apply_room_to_map(&new_room);
            if !map_builder.rooms.is_empty() {
                let (new_x, new_y) = new_room.center();
                let (prev_x, prev_y) = map_builder.rooms[map_builder.rooms.len() - 1].center();
                if rng.range(0, 2) == 1 {
                    map_builder.apply_horizontal_tunnel((prev_x, new_x), prev_y);
                    map_builder.apply_vertical_tunnel((prev_y, new_y), new_x);
                } else {
                    map_builder.apply_horizontal_tunnel((prev_x, new_x), new_y);
                    map_builder.apply_vertical_tunnel((prev_y, new_y), prev_x);
                }
            }
            map_builder.rooms.push(new_room);
        }
    }
    let map = map_builder.build();
    commands.insert_resource(map.clone());
    map
}
