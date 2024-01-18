use ggez::glam::Vec2;
use ggez::graphics::Rect;

use super::util::clamp;

pub struct CameraSettings {
    pub move_sensitivity: f32,
    pub zoom_sensitivity: f32,
    pub min_zoom: f32,
    pub max_zoom: f32,
}

impl Default for CameraSettings {
    fn default() -> Self {
        Self {
            move_sensitivity: 1.0,
            zoom_sensitivity: 0.01,
            min_zoom: 1.0,
            max_zoom: 5.0,
        }
    }
}

pub struct Camera {
    settings: CameraSettings,
    pos: Vec2,
    zoom: f32,
}

impl Default for Camera {
    fn default() -> Self {
        Self::from_settings(CameraSettings::default())
    }
}

impl Camera {
    pub fn new(settings: CameraSettings, pos: Vec2, zoom: f32) -> Self {
        Self {
            settings,
            pos,
            zoom,
        }
    }

    pub fn from_settings(settings: CameraSettings) -> Self {
        Self::new(settings, Vec2::ZERO, 1.0)
    }

    pub fn translate(&mut self, delta: Vec2) {
        self.pos += delta * self.settings.move_sensitivity;
    }

    pub fn zoom(&mut self, delta: f32) {
        self.zoom = clamp(
            self.zoom + delta,
            self.settings.min_zoom,
            self.settings.max_zoom,
        );
    }

    pub fn to_rect(&self) -> Rect {
        Rect::new(self.pos.x, self.pos.y, self.zoom, self.zoom)
    }
}
