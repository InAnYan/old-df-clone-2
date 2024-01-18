use bracket_noise::prelude::FastNoise;
use crate::game::map::{Arr2D, Arr3D, MapSettings};
use crate::game::tile::Tile;

pub fn generate_map(settings: &MapSettings) -> Arr3D<Tile> {
    let noise = FastNoise::seeded(settings.seed);
    let mut data: Arr3D<Tile> = Vec::new();

    for z in 0..settings.depth {
        let mut level: Arr2D<Tile> = Vec::new();

        for y in 0..settings.height {
            let mut row: Vec<Tile> = Vec::new();

            for x in 0..settings.width {
                row.push(generate_tile(settings, &noise, x, y, z));
            }

            level.push(row);
        }

        data.push(level);
    }

    data
}

fn generate_tile(settings: &MapSettings, noise: &FastNoise, x: usize, y: usize, z: usize) -> Tile {
    let terrain_height = settings.depth * 3 / 4;
    let min_terrain_height = settings.depth / 2;
    let dirt_depth = 3;
    let water_level = settings.depth * 62 / 100;

    let noise_float = ((noise.get_noise(x as f32, y as f32) + 1.0) / 2.0)
        * (terrain_height as f32 - min_terrain_height as f32)
        + min_terrain_height as f32;

    let noise = noise_float as usize;

    if z > noise {
        if z > water_level {
            Tile::Air
        } else {
            Tile::Water
        }
    } else if z == noise {
        if noise < water_level {
            Tile::Dirt
        } else {
            Tile::Grass
        }
    } else if z < noise && z > (noise - dirt_depth) {
        Tile::Dirt
    } else {
        Tile::Stone
    }
}
