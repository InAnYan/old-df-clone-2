mod engine;
mod game;

use std::env;
use std::path::PathBuf;

use crate::engine::game_engine::GameEngine;
use crate::game::in_game_screen::InGameScreen;
use crate::game::map::{Map, MapSettings};
use engine::camera::CameraSettings;
use engine::tile_set::TileSet;
use game::tile::GAME_TILE_SIZE;
use ggez::conf::{WindowMode, WindowSetup};
use ggez::glam::Vec2;
use ggez::graphics::Image;
use ggez::{event, ContextBuilder};

const WINDOW_WIDTH: f32 = 800.0;
const WINDOW_HEIGHT: f32 = 600.0;

const WINDOW_TITLE: &'static str = "Dwarf Fortress Clone";
const AUTHOR: &'static str = "InAnYan";

const TILE_SET_TILE_SIZE: Vec2 = Vec2::new(16.0, 16.0);
const TILE_SET_COUNT_X: usize = 5;
const TILE_SET_COUNT_Y: usize = 1;

fn main() {
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        PathBuf::from("./resources")
    };

    let (mut ctx, event_loop) = ContextBuilder::new(WINDOW_TITLE, AUTHOR)
        .window_mode(WindowMode::default().dimensions(WINDOW_WIDTH, WINDOW_HEIGHT))
        .window_setup(WindowSetup::default().title(WINDOW_TITLE))
        .add_resource_path(resource_dir)
        .build()
        .expect("could not create ggez context");

    let tile_set_image = Image::from_path(&ctx, "/tileset1.png").expect("could not load tile set");

    let tile_set = TileSet::new(TILE_SET_COUNT_X, TILE_SET_COUNT_Y, TILE_SET_TILE_SIZE);

    let map = Map::new(
        &ctx,
        MapSettings {
            width: 50,
            height: 50,
            depth: 50,
            seed: 2,
        },
        tile_set_image,
        tile_set,
        GAME_TILE_SIZE,
    );

    let in_game = InGameScreen::new(map, CameraSettings::default());

    let engine = GameEngine::new(&mut ctx, Box::new(in_game));

    event::run(ctx, event_loop, engine);
}
