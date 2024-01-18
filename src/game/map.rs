use crate::engine::tile_set::TileSet;
use crate::game::map_generator::generate_map;
use crate::game::tile::Tile;
use ggez::context::Has;
use ggez::glam::Vec2;
use ggez::graphics::{
    Canvas, Color, DrawParam, Drawable, GraphicsContext, Image, InstanceArray, Rect,
};
use ggez::Context;

pub struct MapSettings {
    pub width: usize,
    pub height: usize,
    pub depth: usize,
    pub seed: u32,
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

    array: InstanceArray,
    tile_set: TileSet,
    game_tile_size: Vec2,
}

impl Map {
    pub fn new(
        ctx: &Context,
        settings: MapSettings,
        image: Image,
        tile_set: TileSet,
        game_tile_size: Vec2,
    ) -> Self {
        Self {
            data: generate_map(&settings),
            level: settings.calculate_start_level(),
            settings,
            array: InstanceArray::new(ctx, image),
            tile_set,
            game_tile_size,
        }
    }

    pub fn level_up(&mut self) {
        if self.level < self.settings.depth - 1 {
            self.level += 1;
            self.regenerate_instance_array();
        }
    }

    pub fn level_down(&mut self) {
        if self.level > 1 {
            self.level -= 1;
            self.regenerate_instance_array();
        }
    }

    pub fn level(&self) -> usize {
        self.level
    }

    pub fn regenerate_instance_array(&mut self) {
        self.array.clear();

        for y in 0..self.settings.height {
            for x in 0..self.settings.width {
                match self.data[self.level][y][x] {
                    Tile::Air => {
                        for new_z in (0..self.level).rev() {
                            match self.data[new_z][y][x] {
                                Tile::Air => continue,
                                tile => {
                                    self.array.push(self.make_tile(tile, x, y, new_z));
                                    break;
                                }
                            }
                        }
                    }

                    tile => {
                        self.array.push(self.make_tile(tile, x, y, self.level));
                    }
                }
            }
        }
    }

    fn make_tile(&self, tile: Tile, x: usize, y: usize, z: usize) -> DrawParam {
        let a = calculate_a(z, self.level, self.settings.depth);

        DrawParam::new()
            .src(self.tile_set.make_src_rect(tile as usize))
            .dest_rect(self.make_rest_rect(x, y))
            .color(Color::new(1.0, 1.0, 1.0, a))
    }

    fn make_rest_rect(&self, x: usize, y: usize) -> Rect {
        Rect::new(
            x as f32 * self.game_tile_size.x,
            y as f32 * self.game_tile_size.y,
            self.game_tile_size.x / self.tile_set.image_tile_size().x,
            self.game_tile_size.y / self.tile_set.image_tile_size().y,
        )
    }
}

fn calculate_a(new: usize, level: usize, count: usize) -> f32 {
    if new == level {
        1.0
    } else {
        ((count - level) as f32) / ((count - new) as f32) / 3.0
    }
}

impl Drawable for Map {
    fn draw(&self, canvas: &mut Canvas, param: impl Into<DrawParam>) {
        self.array.draw(canvas, param);
    }

    fn dimensions(&self, gfx: &impl Has<GraphicsContext>) -> Option<Rect> {
        self.array.dimensions(gfx)
    }
}
