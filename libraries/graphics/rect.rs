use crate::libraries::graphics as gfx;

use super::color::Color;
use super::draw_list::{DrawCommand, DrawTarget};

/// A rectangle, in whatever coordinate space the caller is working in.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Rect {
    pub pos: gfx::FloatPos,
    pub size: gfx::FloatSize,
}

impl Rect {
    #[must_use]
    pub const fn new(pos: gfx::FloatPos, size: gfx::FloatSize) -> Self {
        Self { pos, size }
    }

    /// Records a filled rectangle. Transparent and offscreen ones are dropped here rather than
    /// in the backend: the cheapest command is the one that never reaches the list.
    pub fn render(&self, target: &dyn DrawTarget, color: Color) {
        if color.a == 0 {
            return;
        }

        let draw_area = target.get_draw_area();
        if self.pos.0 > draw_area.0 || self.pos.1 > draw_area.1 || self.pos.0 + self.size.0 < 0.0 || self.pos.1 + self.size.1 < 0.0 {
            return;
        }

        target.push_draw_command(DrawCommand::Rect { rect: *self, color });
    }

    /// Records the rectangle's outline. Not culled offscreen, unlike `render`, so a border
    /// starting off the left edge still draws what is on screen.
    ///
    /// An empty rectangle *is* dropped: it has no edge pixels, and the backend's four quads
    /// would land outside it - the bottom edge sits at `pos.1 + size.1 - 1.0`, a row above the
    /// top one at zero height. A `Button` mid-hover-fade produces one, its hover rectangle
    /// being inset by more than a small button has to give.
    pub fn render_outline(&self, target: &dyn DrawTarget, color: Color) {
        if color.a == 0 || self.size.0 <= 0.0 || self.size.1 <= 0.0 {
            return;
        }

        target.push_draw_command(DrawCommand::RectOutline { rect: *self, color });
    }

    #[must_use]
    pub fn contains(&self, pos: gfx::FloatPos) -> bool {
        pos.0 >= self.pos.0 && pos.0 <= self.pos.0 + self.size.0 && pos.1 >= self.pos.1 && pos.1 <= self.pos.1 + self.size.1
    }
}
