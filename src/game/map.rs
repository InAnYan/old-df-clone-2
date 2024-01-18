use ggez::context::Has;
use ggez::graphics::{Canvas, Drawable, DrawParam, GraphicsContext, Rect};
use crate::engine::tile_map::TileMap;
use crate::game::map_generator::generate_map;
use crate::game::tile::Tile;

pub struct MapSettings {
    pub width: usize,
    pub height: usize,
    pub depth: usize,
    pub seed: u64,
}

impl MapSettings {
    pub fn calculate_start_level(&self) -> usize {
        self.depth * 3 / 4
    }
}

pub type Arr2D<T> = Vec<Vec<T>>;
pub type Arr3D<T> = Vec<Arr2D<T>>;

pub struct Map {
    data: Arr3D<Tile>,
    level: usize,
    settings: MapSettings,
    tile_map: TileMap,
}

impl Map {
    pub fn new(settings: MapSettings, tile_map: TileMap) -> Self {
        Self {
            data: generate_map(&settings),
            level: settings.calculate_start_level(),
            settings,
            tile_map,
        }
    }

    pub fn level_up(&mut self) {
        if self.level < self.settings.depth - 1 {
            self.level += 1;
            self.refill_map();
        }
    }

    pub fn level_down(&mut self) {
        if self.level > 1 {
            self.level -= 1;
            self.refill_map();
        }
    }

    pub fn level(&self) -> usize {
        self.level
    }

    fn refill_map(&mut self) {
        self.tile_map.refill(&self.data[self.level]);
    }
}

impl Drawable for Map {
    fn draw(&self, canvas: &mut Canvas, param: impl Into<DrawParam>) {
       self.tile_map.draw(canvas, param);
    }

    fn dimensions(&self, gfx: &impl Has<GraphicsContext>) -> Option<Rect> {
        self.tile_map.dimensions(gfx)
    }
}