use bevy::prelude::{Commands, Plugin, Res};
use bracket_bevy::{
    prelude::{to_cp437, RGB},
    BracketContext, RandomNumbers,
};

#[derive(PartialEq, Copy, Clone)]
enum TileType {
    Wall,
    Floor,
}

pub struct Map(Vec<TileType>);

impl Map {
    pub fn new() -> Self {
        let mut map = vec![TileType::Floor; 80 * 50];
        // Make the boundaries walls
        for x in 0..80 {
            map[Self::xy_idx(x, 0)] = TileType::Wall;
            map[Self::xy_idx(x, 49)] = TileType::Wall;
        }
        for y in 0..50 {
            map[Self::xy_idx(0, y)] = TileType::Wall;
            map[Self::xy_idx(79, y)] = TileType::Wall;
        }

        // Now we'll randomly splat a bunch of walls. It won't be pretty, but it's a decent illustration.
        // First, obtain the thread-local RNG:
        let rng = RandomNumbers::new();

        for _i in 0..400 {
            let x = rng.roll_dice(1, 79);
            let y = rng.roll_dice(1, 49);
            let idx = Self::xy_idx(x, y);
            if idx != Self::xy_idx(40, 25) {
                map[idx] = TileType::Wall;
            }
        }
        Self(map)
    }
    pub fn xy_idx(x: i32, y: i32) -> usize {
        (y as usize * 80) + x as usize
    }
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

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_startup_system(setup);
    }
}

fn setup(mut commands: Commands) {
    commands.insert_resource(Map::new());
}
