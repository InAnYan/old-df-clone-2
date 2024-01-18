use crate::engine::tile_set::TileSet;
use crate::game::map::Arr2D;
use ggez::context::Has;
use ggez::glam::Vec2;
use ggez::graphics::{Canvas, DrawParam, Drawable, GraphicsContext, Image, InstanceArray, Rect};
use ggez::Context;

pub struct TileMap {
    array: InstanceArray,
    tile_set: TileSet,
    game_tile_size: Vec2,
}

impl TileMap {
    pub fn new(ctx: &Context, image: Image, tile_set: TileSet, game_tile_size: Vec2) -> Self {
        Self {
            array: InstanceArray::new(ctx, Some(image)),
            tile_set,
            game_tile_size,
        }
    }

    pub fn refill<T: Into<usize> + Copy>(&mut self, data: &Arr2D<T>) {
        self.array.clear();

        for (y, row) in data.iter().enumerate() {
            for (x, tile) in row.iter().enumerate() {
                let param = DrawParam::new()
                    .src(self.tile_set.make_src_rect((*tile).into()))
                    .dest_rect(self.make_rest_rect(x, y));
                self.array.push(param)
            }
        }
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

impl Drawable for TileMap {
    fn draw(&self, canvas: &mut Canvas, param: impl Into<DrawParam>) {
        self.array.draw(canvas, param)
    }

    fn dimensions(&self, gfx: &impl Has<GraphicsContext>) -> Option<Rect> {
        self.array.dimensions(gfx)
    }
}
