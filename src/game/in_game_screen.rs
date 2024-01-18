use crate::engine::camera::{Camera, CameraSettings};
use crate::engine::events::Event;
use crate::engine::screen::Screen;
use crate::engine::screen_result::{ScreenEvent, ScreenResult};
use crate::game::map::Map;
use ggegui::{egui, GuiContext};
use ggez::event::MouseButton;
use ggez::graphics::{Canvas, DrawParam, Drawable};
use ggez::{Context, GameResult};

pub struct InGameScreen {
    map: Map,
    camera: Camera,
    rmb_down: bool,
}

impl InGameScreen {
    pub fn new(map: Map, camera_settings: CameraSettings) -> Self {
        Self {
            map,
            camera: Camera::from_settings(camera_settings),
            rmb_down: false,
        }
    }
}

impl Screen for InGameScreen {
    fn input(&mut self, _ctx: &mut Context, event: Event) -> ScreenResult {
        match event {
            Event::MouseMotion { pos: _, delta } => {
                if self.rmb_down {
                    self.camera.translate(delta);
                }
            }

            Event::MouseButtonDown(button) => {
                if button == MouseButton::Right {
                    self.rmb_down = true;
                }
            }

            Event::MouseButtonUp(button) => {
                if button == MouseButton::Right {
                    self.rmb_down = false;
                }
            }

            Event::MouseWheel(vec) => {
                self.camera.zoom(vec.y);
            }
        }

        Ok(ScreenEvent::Stay)
    }

    fn update(&mut self, _ctx: &mut Context, gui_ctx: &GuiContext) -> ScreenResult {
        egui::Window::new("Level").show(gui_ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("<").clicked() {
                    self.map.level_down();
                }

                ui.label(self.map.level().to_string());

                if ui.button(">").clicked() {
                    self.map.level_up();
                }
            });
        });

        Ok(ScreenEvent::Stay)
    }

    fn draw(&mut self, canvas: &mut Canvas) -> GameResult {
        self.map
            .draw(canvas, DrawParam::new().dest_rect(self.camera.to_rect()));
        Ok(())
    }
}
