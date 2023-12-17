use crate::libraries::graphics as gfx;

use super::color::Color;
use super::vertex_buffer::DrawMode;

/// This is a rectangle shape.
#[derive(Clone, Copy)]
pub struct Rect {
    pub pos: gfx::FloatPos,
    pub size: gfx::FloatSize,
}

impl Rect {
    /// Creates a new rectangle.
    #[must_use]
    pub const fn new(pos: gfx::FloatPos, size: gfx::FloatSize) -> Self {
        Self { pos, size }
    }

    /// Renders the rectangle on the screen.
    pub fn render(&self, graphics: &gfx::GraphicsContext, color: Color) {
        if color.a == 0 {
            return;
        }

        if self.pos.0 > graphics.get_window_size().0 || self.pos.1 > graphics.get_window_size().1 || self.pos.0 + self.size.0 < 0.0 || self.pos.1 + self.size.1 < 0.0 {
            return;
        }

        let mut transform = graphics.normalization_transform.clone();
        transform.translate(self.pos);
        transform.stretch((self.size.0, self.size.1));

        // Safety: We are using a valid shader.
        unsafe {
            gl::UniformMatrix3fv(graphics.passthrough_shader.transform_matrix, 1, gl::FALSE, &transform.matrix[0]);
            gl::Uniform4f(
                graphics.passthrough_shader.global_color,
                color.r as f32 / 255.0,
                color.g as f32 / 255.0,
                color.b as f32 / 255.0,
                color.a as f32 / 255.0,
            );
            gl::Uniform1i(graphics.passthrough_shader.has_texture, 0);
        }

        graphics.passthrough_shader.rect_vertex_buffer.draw(false, DrawMode::Triangles);
    }

    /// Renders the rectangle outline on the screen.
    pub fn render_outline(&self, graphics: &gfx::GraphicsContext, color: Color) {
        if color.a == 0 {
            return;
        }

        let mut transform = graphics.normalization_transform.clone();
        transform.translate(self.pos);
        transform.stretch((self.size.0, self.size.1));

        // Safety: We are using a valid shader.
        unsafe {
            gl::UniformMatrix3fv(graphics.passthrough_shader.transform_matrix, 1, gl::FALSE, &transform.matrix[0]);
            gl::Uniform4f(
                graphics.passthrough_shader.global_color,
                color.r as f32 / 255.0,
                color.g as f32 / 255.0,
                color.b as f32 / 255.0,
                color.a as f32 / 255.0,
            );
            gl::Uniform1i(graphics.passthrough_shader.has_texture, 0);
        }

        graphics.passthrough_shader.rect_outline_vertex_buffer.draw(false, DrawMode::Lines);
    }

    #[must_use]
    pub fn contains(&self, pos: gfx::FloatPos) -> bool {
        pos.0 >= self.pos.0 && pos.0 <= self.pos.0 + self.size.0 && pos.1 >= self.pos.1 && pos.1 <= self.pos.1 + self.size.1
    }
}
