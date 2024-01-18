use ggez::GameError;
use crate::engine::screen::Screen;

pub enum ScreenEvent {
    Stay,
    Change(Box<dyn Screen>),
    Exit,
}

pub type ScreenResult = Result<ScreenEvent, GameError>;
