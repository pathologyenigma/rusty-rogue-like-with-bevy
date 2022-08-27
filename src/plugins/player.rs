use bevy::prelude::{Commands, Input, KeyCode, Plugin, Query, Res, With};
use bracket_bevy::prelude::{to_cp437, BLACK, RGB, YELLOW};

use crate::components::{Player, Position, Renderable};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_startup_system(setup).add_system(move_player);
    }
}

fn setup(mut commands: Commands) {
    commands
        .spawn()
        .insert(Position { x: 40, y: 25 })
        .insert(Renderable {
            glyph: to_cp437('@'),
            fg: RGB::named(YELLOW),
            bg: RGB::named(BLACK),
        })
        .insert(Player);
}

fn move_player(keys: Res<Input<KeyCode>>, mut query: Query<&mut Position, With<Player>>) {
    let mut delta: Position = Default::default();
    let mut player = query.single_mut();
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
    *player += delta;
}
