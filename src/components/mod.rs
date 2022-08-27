use std::ops::AddAssign;

use bevy::prelude::Component;
use bracket_bevy::prelude::RGB;

#[derive(Component)]
pub(crate) struct Renderable {
    pub(crate) glyph: bracket_bevy::FontCharType,
    pub(crate) fg: RGB,
    pub(crate) bg: RGB,
}
#[derive(Component, Debug)]
pub(crate) struct Player;

#[derive(Component, Default)]
pub(crate) struct Position {
    pub(crate) x: i32,
    pub(crate) y: i32,
}
impl AddAssign for Position {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}
