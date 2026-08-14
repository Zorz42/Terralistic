use crate::libraries::graphics as gfx;

use super::UiElement;
use gfx::Rect;

#[derive(Clone, Copy, Debug)]
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

/// A rectangle positioned relative to a parent by an orientation plus an offset.
///
/// The orientation is the fraction of the parent the container is offset by, *and* the
/// fraction of its own size it is pulled back by - so `CENTER` centres the element rather than
/// putting its top left corner in the middle. With no parent the window is the parent.
pub struct Container {
    pub rect: Rect,
    abs_rect: Rect,
    pub orientation: Orientation,
}

impl Container {
    #[must_use]
    pub fn new(graphics: &dyn super::UiContext, pos: gfx::FloatPos, size: gfx::FloatSize, orientation: Orientation, parent_container: Option<&Self>) -> Self {
        let mut result = Self {
            rect: Rect::new(pos, size),
            abs_rect: Rect::new(gfx::FloatPos(0.0, 0.0), gfx::FloatSize(0.0, 0.0)),
            orientation,
        };
        result.update_position(graphics, parent_container);
        result
    }

    #[must_use]
    pub fn default(graphics_context: &dyn super::UiContext) -> Self {
        Self::new(graphics_context, gfx::FloatPos(0.0, 0.0), graphics_context.get_window_size(), TOP_LEFT, None)
    }

    /// The container's rectangle in window coordinates, as of the last layout.
    #[must_use]
    pub const fn get_absolute_rect(&self) -> &Rect {
        &self.abs_rect
    }

    fn update_position(&mut self, graphics: &dyn super::UiContext, parent_container: Option<&Self>) {
        let parent_rect = parent_container.map_or_else(|| Rect::new(gfx::FloatPos(0.0, 0.0), graphics.get_window_size()), |parent| *parent.get_absolute_rect());

        self.abs_rect.pos = parent_rect.pos + self.rect.pos + gfx::FloatPos(parent_rect.size.0 * self.orientation.x, parent_rect.size.1 * self.orientation.y)
            - gfx::FloatPos(self.rect.size.0 * self.orientation.x, self.rect.size.1 * self.orientation.y);
        self.abs_rect.size = self.rect.size;
    }
}

impl UiElement for Container {
    /// Recomputes the absolute rectangle against the parent.
    fn update_inner(&mut self, graphics: &mut gfx::GraphicsContext, parent_container: &Self) {
        self.update_position(graphics, Some(parent_container));
    }

    fn get_container(&self, graphics: &dyn super::UiContext, parent_container: &Container) -> Container {
        Self::new(graphics, self.rect.pos, self.rect.size, self.orientation, Some(parent_container))
    }
}
