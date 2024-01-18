use noise::{NoiseFn, Perlin};

pub struct NoiseGenerator {
    perlin: Perlin,
    scale: f32,
    offset: f32,
}

impl NoiseGenerator {
    pub fn new(seed: u32, scale: f32, offset: f32) -> NoiseGenerator {
        NoiseGenerator {
            perlin: Perlin::new(seed),
            scale,
            offset,
        }
    }

    pub fn get(&self, x: f32, y: f32) -> f32 {
        self.perlin.get([
            (x * self.scale + self.offset) as f64,
            (y * self.scale + self.offset) as f64,
        ]) as f32
    }
}
