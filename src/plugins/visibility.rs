use bevy::prelude::{Plugin, Query, ResMut};
use bracket_algorithm_traits::prelude::{Algorithm2D, BaseMap};
use bracket_bevy::prelude::Point;
use bracket_pathfinding::prelude::field_of_view;

use crate::{
    components::{Position, ViewShed, Player},
    res::{Map, TileType, xy_idx},
};

pub struct VisibilityPlugin;

impl Plugin for VisibilityPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system(visibility);
    }
}

fn visibility(mut query: Query<(&mut ViewShed, &Position, Option<&Player>)>, mut map: ResMut<Map>) {
    for (mut viewshed, pos, player) in query.iter_mut() {
        if viewshed.dirty {
            viewshed.dirty = false;
            viewshed.visible_tiles.clear();
            viewshed.visible_tiles =
                field_of_view(Point::new(pos.x, pos.y), viewshed.range, map.as_ref());
            viewshed
                .visible_tiles
                .retain(|p| p.x >= 0 && p.x < map.width && p.y >= 0 && p.y < map.height);
            if let Some(_p) = player {
                for t in map.visible_tiles.iter_mut() {*t = false}
                for vis in viewshed.visible_tiles.iter() {
                    let idx = xy_idx(vis.x, vis.y);
                    map.revealed_tiles[idx] = true;
                    map.visible_tiles[idx] = true;
                }
            }
        }
        
    }
}

impl BaseMap for Map {
    fn is_opaque(&self, idx: usize) -> bool {
        self.tiles[idx as usize] == TileType::Wall
    }
}

impl Algorithm2D for Map {
    fn dimensions(&self) -> bracket_bevy::prelude::Point {
        Point::new(self.width, self.height)
    }
}
