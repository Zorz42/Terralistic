use super::color::Color;
use super::vertex_buffer::DrawMode;
use super::GraphicsContext;

/**
This is a rectangle shape not really
meant to be rendered.
 */
#[derive(Clone, Copy)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
}

impl Rect {
    /**
    Creates a new rectangle.
     */
    #[must_use]
    pub fn new(x: i32, y: i32, w: i32, h: i32) -> Self {
        Self { x, y, w, h }
    }

    /**
    Renders the rectangle on the screen.
     */
    pub fn render(&self, graphics: &GraphicsContext, color: Color) {
        if color.a == 0 {
            return;
        }

        let mut transform = graphics.renderer.normalization_transform.clone();
        transform.translate(self.x as f32, self.y as f32);
        transform.stretch(self.w as f32, self.h as f32);

        unsafe {
            gl::UniformMatrix3fv(
                graphics.renderer.passthrough_shader.transform_matrix,
                1,
                gl::FALSE,
                &transform.matrix[0],
            );
            gl::Uniform4f(
                graphics.renderer.passthrough_shader.global_color,
                color.r as f32 / 255.0,
                color.g as f32 / 255.0,
                color.b as f32 / 255.0,
                color.a as f32 / 255.0,
            );
            gl::Uniform1i(graphics.renderer.passthrough_shader.has_texture, 0);
        }

        graphics
            .renderer
            .passthrough_shader
            .rect_vertex_buffer
            .draw(false, DrawMode::Triangles);
    }

    /**
    Renders the rectangle outline on the screen.
     */
    pub fn render_outline(&self, graphics: &GraphicsContext, color: Color) {
        if color.a == 0 {
            return;
        }

        let mut transform = graphics.renderer.normalization_transform.clone();
        transform.translate(self.x as f32, self.y as f32);
        transform.stretch(self.w as f32, self.h as f32);

        unsafe {
            gl::UniformMatrix3fv(
                graphics.renderer.passthrough_shader.transform_matrix,
                1,
                gl::FALSE,
                &transform.matrix[0],
            );
            gl::Uniform4f(
                graphics.renderer.passthrough_shader.global_color,
                color.r as f32 / 255.0,
                color.g as f32 / 255.0,
                color.b as f32 / 255.0,
                color.a as f32 / 255.0,
            );
            gl::Uniform1i(graphics.renderer.passthrough_shader.has_texture, 0);
        }

        graphics
            .renderer
            .passthrough_shader
            .rect_outline_vertex_buffer
            .draw(false, DrawMode::Lines);
    }
}
