mod rect;
use bevy::prelude::{Commands, Res};
use bracket_bevy::{
    prelude::{to_cp437, RGB},
    BracketContext, RandomNumbers,
};
use rect::Rect;

#[derive(PartialEq, Copy, Clone)]
enum TileType {
    Wall,
    Floor,
}
#[derive(Clone)]
pub struct Map(Vec<TileType>);

pub fn xy_idx(x: i32, y: i32) -> usize {
    (y as usize * 80) + x as usize
}

impl Map {
    pub fn draw(&self, ctx: &Res<BracketContext>) {
        let (mut x, mut y) = Default::default();
        for tile in self.0.iter() {
            // Render a tile depending upon the tile type
            match tile {
                TileType::Floor => {
                    ctx.set(
                        x,
                        y,
                        RGB::from_f32(0.5, 0.5, 0.5),
                        RGB::from_f32(0., 0., 0.),
                        to_cp437('.'),
                    );
                }
                TileType::Wall => {
                    ctx.set(
                        x,
                        y,
                        RGB::from_f32(0., 1., 0.),
                        RGB::from_f32(0., 0., 0.),
                        to_cp437('#'),
                    );
                }
            }
            x += 1;
            if x > 79 {
                x = 0;
                y += 1;
            }
        }
    }
}

struct MapBuilder {
    tiles: Vec<TileType>,
}

impl Default for MapBuilder {
    fn default() -> Self {
        Self {
            tiles: vec![TileType::Wall; 80 * 50],
        }
    }
}
impl MapBuilder {
    fn build(self) -> Map {
        Map(self.tiles)
    }
    fn apply_room_to_map(&mut self, room: &Rect) {
        for y in room.y.0 + 1..room.y.1 {
            for x in room.x.0 + 1..room.x.1 {
                self.tiles[xy_idx(x, y)] = TileType::Floor;
            }
        }
    }
    fn apply_horizontal_tunnel(&mut self, x: (i32, i32), y: i32) {
        for x in x.0.min(x.1)..x.0.max(x.1) {
            let idx = xy_idx(x, y);
            if idx > 0 && idx < self.tiles.len() {
                self.tiles[idx as usize] = TileType::Floor;
            }
        }
    }
    fn apply_vertical_tunnel(&mut self, y: (i32, i32), x: i32) {
        for y in y.0.min(y.1)..y.0.max(y.1) {
            let idx = xy_idx(x, y);
            if idx > 0 && idx < self.tiles.len() {
                self.tiles[idx as usize] = TileType::Floor;
            }
        }
    }
}

#[derive(Clone)]
pub struct Rooms(pub Vec<Rect>);

pub fn setup(commands: &mut Commands) -> (Map, Rooms){
    let mut rooms = Vec::new();
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
        for other_room in rooms.iter() {
            if new_room.intersect(other_room) {
                ok = false
            }
        }
        if ok {
            map_builder.apply_room_to_map(&new_room);
            if !rooms.is_empty() {
                let (new_x, new_y) = new_room.center();
                let (prev_x, prev_y) = rooms[rooms.len() - 1].center();
                if rng.range(0, 2) == 1{
                    map_builder.apply_horizontal_tunnel((prev_x, new_x), prev_y);
                    map_builder.apply_vertical_tunnel((prev_y, new_y), new_x);
                } else {
                    map_builder.apply_horizontal_tunnel((prev_x, new_x), new_y);
                    map_builder.apply_vertical_tunnel((prev_y, new_y), prev_x);
                }
            }
            rooms.push(new_room);
        }
    }
    let map = map_builder.build();
    commands.insert_resource(map.clone());
    let rooms = Rooms(rooms);
    commands.insert_resource(rooms.clone());
    (map, rooms)
}
