use super::Rect;
use crate::libraries::graphics::GraphicsContext;

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
    pub const fn new(x: i32, y: i32, w: i32, h: i32, orientation: Orientation) -> Self {
        Self {
            rect: Rect::new(x, y, w, h),
            abs_rect: Rect::new(x, y, w, h),
            orientation,
        }
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
    pub fn update(&mut self, graphics: &GraphicsContext, parent_container: Option<&Self>) {
        let parent_rect = parent_container.map_or_else(
            || {
                Rect::new(
                    0,
                    0,
                    graphics.renderer.get_window_width() as i32,
                    graphics.renderer.get_window_height() as i32,
                )
            },
            |parent| *parent.get_absolute_rect(),
        );

        self.abs_rect = self.rect;
        self.abs_rect.x =
            parent_rect.x + self.rect.x + (parent_rect.w as f32 * self.orientation.x) as i32
                - (self.rect.w as f32 * self.orientation.x) as i32;
        self.abs_rect.y =
            parent_rect.y + self.rect.y + (parent_rect.h as f32 * self.orientation.y) as i32
                - (self.rect.h as f32 * self.orientation.y) as i32;
    }
}
