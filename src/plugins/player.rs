use bevy::prelude::{Commands, Input, KeyCode, Plugin, Query, Res, With};
use bracket_bevy::prelude::{to_cp437, BLACK, RGB, YELLOW};

use crate::{components::{Player, Position, Renderable, ViewShed}, res::{self, Map, xy_idx, TileType}};


pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
        .add_startup_system(setup)
        .add_system(move_player);
    }
}

fn setup(mut commands: Commands) {
    let(x,y) = res::map_setup(&mut commands).rooms[0].center();
    commands
    .spawn()
    .insert(Position { x, y })
    .insert(Renderable {
        glyph: to_cp437('@'),
        fg: RGB::named(YELLOW),
        bg: RGB::named(BLACK),
    })
    .insert(Player)
    .insert(ViewShed{
        visible_tiles: Vec::new(),
        range: 8,
        dirty: true
    });
}

fn move_player(keys: Res<Input<KeyCode>>, mut query: Query<(&mut Position, &mut ViewShed), With<Player>>, map: Res<Map>) {
    let mut delta: Position = Default::default();
    let (mut pos, mut viewshed) = query.single_mut();
    if keys.just_pressed(KeyCode::Left) {
        delta += Position { x: -1, y: 0 };
    }
    if keys.just_pressed(KeyCode::Right) {
        delta += Position { x: 1, y: 0 };
    }
    if keys.just_pressed(KeyCode::Up) {
        delta += Position { x: 0, y: -1 };
    }
    if keys.just_pressed(KeyCode::Down) {
        delta += Position { x: 0, y: 1 };
    }
    let destination_idx = xy_idx(pos.x + delta.x, pos.y + delta.y);
    if map.tiles[destination_idx] != TileType::Wall {
        pos.x = 79.min((pos.x + delta.x).max(0));
        pos.y = 49.min((pos.y + delta.y).max(0));

        viewshed.dirty = true;
    }
}

