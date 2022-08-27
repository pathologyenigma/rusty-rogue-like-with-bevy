pub mod components;
mod plugins;
mod res;
use components::{Position, Renderable};
use plugins::prelude::*;

use bevy::prelude::*;
use bracket_bevy::prelude::*;
use res::Map;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(BTermBuilder::simple_80x50())
        .add_plugin(PlayerPlugin)
        .add_system(tick)
        .run();
}

fn tick(ctx: Res<BracketContext>, query: Query<(&Position, &Renderable)>, map: Res<Map>) {
    ctx.cls();
    map.draw(&ctx);
    for (pos, render) in query.iter() {
        ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
    }
}

