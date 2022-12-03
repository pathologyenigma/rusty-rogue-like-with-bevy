pub mod components;
mod plugins;
mod res;
use components::{Position, Renderable};
use plugins::prelude::*;

use bevy::prelude::*;
use bracket_bevy::prelude::*;
use res::{Map, TileType};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(BTermBuilder::simple_80x50())
        .add_plugin(PlayerPlugin)
        .add_plugin(VisibilityPlugin)
        .add_system(tick)
        .run();
}

fn tick(
    ctx: Res<BracketContext>,
    query: Query<(&Position, &Renderable)>,
    map: Res<Map>,
) {
    ctx.cls();
    draw(&map, &ctx);
    for (pos, render) in query.iter() {
        ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
    }
}

fn draw(map: &Res<Map>, ctx: &Res<BracketContext>) {
    let (mut x, mut y) = Default::default();
    for (idx, tile) in map.tiles.iter().enumerate() {
        if map.revealed_tiles[idx] {
            let glyph;
            let mut fg;
            match tile {
                TileType::Floor => {
                    glyph = to_cp437('.');
                    fg = RGB::from_f32(0., 0.5, 0.5);
                }
                TileType::Wall => {
                    glyph = to_cp437('#');
                    fg = RGB::from_f32(0., 1., 0.);
                }
            }
            if !map.visible_tiles[idx] {fg = fg.to_greyscale()}
            ctx.set(x, y, fg, RGB::from_f32(0.,0., 0.), glyph);
        }
        x += 1;
        if x > 79 {
            x = 0;
            y += 1;
        }
    }
}
