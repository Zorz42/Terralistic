use crate::libraries::graphics as gfx;

use super::Rect;

#[derive(Clone, Copy)]
pub struct Orientation {
    pub x: f32,
    pub y: f32,
}

pub const TOP_LEFT: Orientation = Orientation { x: 0.0, y: 0.0 };
pub const TOP: Orientation = Orientation { x: 0.5, y: 0.0 };
pub const TOP_RIGHT: Orientation = Orientation { x: 1.0, y: 0.0 };
pub const LEFT: Orientation = Orientation { x: 0.0, y: 0.5 };
pub const CENTER: Orientation = Orientation { x: 0.5, y: 0.5 };
pub const RIGHT: Orientation = Orientation { x: 1.0, y: 0.5 };
pub const BOTTOM_LEFT: Orientation = Orientation { x: 0.0, y: 1.0 };
pub const BOTTOM: Orientation = Orientation { x: 0.5, y: 1.0 };
pub const BOTTOM_RIGHT: Orientation = Orientation { x: 1.0, y: 1.0 };

/// This struct has coordinates and size of a rectangle.
/// Is has orientation and parent container. It also has
/// a function to get absolute value of the rectangle.
pub struct Container {
    pub rect: Rect,
    abs_rect: Rect,
    pub orientation: Orientation,
}

impl Container {
    /// Creates a new container.
    #[must_use]
    pub fn new(graphics: &gfx::GraphicsContext, pos: gfx::FloatPos, size: gfx::FloatSize, orientation: Orientation, parent_container: Option<&Self>) -> Self {
        let mut result = Self {
            rect: Rect::new(pos, size),
            abs_rect: Rect::new(gfx::FloatPos(0.0, 0.0), gfx::FloatSize(0.0, 0.0)),
            orientation,
        };
        result.update(graphics, parent_container);
        result
    }

    /// Returns the absolute rectangle of the container. Orientation
    /// is a percentage the container is offset from the top left corner
    /// of the parent container. If parent is None, the parent is the
    /// window. This function needs `graphics_context` to get the window size.
    #[must_use]
    pub const fn get_absolute_rect(&self) -> &Rect {
        &self.abs_rect
    }

    /// This function gets parent container and updates the absolute values
    pub fn update(&mut self, graphics: &gfx::GraphicsContext, parent_container: Option<&Self>) {
        let parent_rect = parent_container.map_or_else(|| Rect::new(gfx::FloatPos(0.0, 0.0), graphics.renderer.get_window_size()), |parent| *parent.get_absolute_rect());

        self.abs_rect.pos = parent_rect.pos + self.rect.pos + gfx::FloatPos(parent_rect.size.0 * self.orientation.x, parent_rect.size.1 * self.orientation.y)
            - gfx::FloatPos(self.rect.size.0 * self.orientation.x, self.rect.size.1 * self.orientation.y);
        self.abs_rect.size = self.rect.size;
    }
}
