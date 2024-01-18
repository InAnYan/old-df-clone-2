use crate::engine::events::Event;
use crate::engine::screen::Screen;
use crate::engine::screen_result::ScreenEvent;
use ggegui::Gui;
use ggez::event::{EventHandler, MouseButton};
use ggez::glam::Vec2;
use ggez::graphics::{Canvas, Color, DrawParam, Sampler};
use ggez::{Context, GameError, GameResult};

pub struct GameEngine {
    current_screen: Box<dyn Screen>,
    gui: Gui,
}

impl GameEngine {
    pub fn new(ctx: &mut Context, screen: Box<dyn Screen>) -> GameEngine {
        GameEngine {
            current_screen: screen,
            gui: Gui::new(ctx),
        }
    }

    fn screen_input(&mut self, ctx: &mut Context, event: Event) -> GameResult {
        let result = self.current_screen.input(ctx, event)?;

        self.process_screen_event(ctx, result);

        Ok(())
    }

    fn process_screen_event(&mut self, ctx: &mut Context, result: ScreenEvent) {
        match result {
            ScreenEvent::Stay => {}

            ScreenEvent::Change(new_screen) => {
                self.current_screen = new_screen;
            }

            ScreenEvent::Exit => {
                ctx.request_quit();
            }
        }
    }
}

impl EventHandler for GameEngine {
    fn mouse_motion_event(
        &mut self,
        ctx: &mut Context,
        x: f32,
        y: f32,
        dx: f32,
        dy: f32,
    ) -> Result<(), GameError> {
        self.screen_input(
            ctx,
            Event::MouseMotion {
                pos: Vec2::new(x, y),
                delta: Vec2::new(dx, dy),
            },
        )
    }

    fn mouse_button_down_event(
        &mut self,
        ctx: &mut Context,
        button: MouseButton,
        _x: f32,
        _y: f32,
    ) -> Result<(), GameError> {
        self.screen_input(ctx, Event::MouseButtonDown(button))
    }

    fn mouse_button_up_event(
        &mut self,
        ctx: &mut Context,
        button: MouseButton,
        _x: f32,
        _y: f32,
    ) -> Result<(), GameError> {
        self.screen_input(ctx, Event::MouseButtonUp(button))
    }

    fn mouse_wheel_event(&mut self, ctx: &mut Context, x: f32, y: f32) -> Result<(), GameError> {
        self.screen_input(ctx, Event::MouseWheel(Vec2::new(x, y)))
    }

    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let gui_ctx = self.gui.ctx();

        let result = self.current_screen.update(ctx, &gui_ctx)?;

        self.process_screen_event(ctx, result);

        self.gui.update(ctx);

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = Canvas::from_frame(ctx, Color::BLACK);

        canvas.set_sampler(Sampler::nearest_clamp());

        self.current_screen.draw(&mut canvas)?;

        canvas.draw(&self.gui, DrawParam::default().dest(Vec2::ZERO));

        canvas.finish(ctx)
    }
}
