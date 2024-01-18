use crate::engine::screen::Screen;

pub enum GameResult {
    Stay,
    Change(Box<dyn Screen>),
    Exit,
}
