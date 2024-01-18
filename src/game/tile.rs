use ggez::glam::Vec2;

pub const GAME_TILE_SIZE: Vec2 = Vec2::new(16.0, 16.0);

#[derive(Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
pub enum Tile {
    Air = 0,
    Grass,
    Dirt,
    Stone,
    Water
}

impl Into<usize> for Tile {
    fn into(self) -> usize {
        self as usize
    }
}

// TODO: Descriptions on mouse.
pub fn _tile_description(tile: Tile) -> &'static str {
    match tile {
        Tile::Air => "air",
        Tile::Dirt => "dirt",
        Tile::Grass => "grass",
        Tile::Stone => "stone",
        Tile::Water => "water",
    }
}
