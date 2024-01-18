use ggegui::GuiContext;
use ggez::{Context, GameResult};
use ggez::graphics::Canvas;
use crate::engine::events::Event;
use crate::engine::screen_result::ScreenResult;

pub trait Screen {
    fn input(&mut self, ctx: &mut Context, event: Event) -> ScreenResult;
    fn update(&mut self, ctx: &mut Context, gui_ctx: &GuiContext) -> ScreenResult;
    fn draw(&mut self, canvas: &mut Canvas) -> GameResult;
}