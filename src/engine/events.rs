use ggez::event::MouseButton;
use ggez::glam::Vec2;

pub enum Event {
    MouseMotion { pos: Vec2, delta: Vec2 },
    MouseButtonDown(MouseButton),
    MouseButtonUp(MouseButton),
    MouseWheel(Vec2),
}
