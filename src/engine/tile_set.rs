use ggez::glam::Vec2;
use ggez::graphics::Rect;

pub struct TileSet {
    tile_count_x: usize,
    tile_count_y: usize,
    image_tile_size: Vec2,
}

impl TileSet {
    pub fn new(
        tile_count_x: usize,
        tile_count_y: usize,
        image_tile_size: Vec2,
    ) -> Self {
        Self {
            tile_count_x,
            tile_count_y,
            image_tile_size,
        }
    }

    pub fn make_src_rect(&self, id: usize) -> Rect {
        let x = id % self.tile_count_x;
        let y = id / self.tile_count_x;

        assert!(x < self.tile_count_x);
        assert!(y < self.tile_count_y);

        Rect::new(
            x as f32 / self.tile_count_x as f32, y as f32 / self.tile_count_y as f32,
            1.0 / self.tile_count_x as f32, 1.0 / self.tile_count_y as f32
        )
    }

    pub fn image_tile_size(&self) -> Vec2 {
        self.image_tile_size
    }
}

