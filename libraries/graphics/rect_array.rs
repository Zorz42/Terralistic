use crate::libraries::graphics as gfx;

use super::vertex_buffer::{DrawMode, Vertex, VertexBuffer};

/// The struct `RectArray` is used to draw multiple rectangles with the same texture
/// and in one draw call. This is much faster than drawing each rectangle individually.
pub struct RectArray {
    vertex_buffer: VertexBuffer,
}

impl RectArray {
    /// Creates a new `RectArray`.
    #[must_use]
    pub fn new() -> Self {
        Self {
            vertex_buffer: VertexBuffer::new(),
        }
    }

    /// Adds a rectangle to the `RectArray`.
    pub fn add_rect(&mut self, rect: &gfx::Rect, colors: &[gfx::Color; 4], tex_rect: &gfx::Rect) {
        let top_left = rect.pos;
        let top_right = rect.pos + gfx::FloatSize(rect.size.0, 0.0);
        let bottom_left = rect.pos + gfx::FloatSize(0.0, rect.size.1);
        let bottom_right = rect.pos + rect.size;

        let tex_top_left = tex_rect.pos;
        let tex_top_right = tex_rect.pos + gfx::FloatSize(tex_rect.size.0, 0.0);
        let tex_bottom_left = tex_rect.pos + gfx::FloatSize(0.0, tex_rect.size.1);
        let tex_bottom_right = tex_rect.pos + tex_rect.size;

        // first triangle
        self.vertex_buffer.add_vertex(&Vertex {
            pos: top_left,
            color: colors[0],
            tex_pos: tex_top_left,
        });

        self.vertex_buffer.add_vertex(&Vertex {
            pos: top_right,
            color: colors[1],
            tex_pos: tex_top_right,
        });

        self.vertex_buffer.add_vertex(&Vertex {
            pos: bottom_left,
            color: colors[2],
            tex_pos: tex_bottom_left,
        });

        // second triangle
        self.vertex_buffer.add_vertex(&Vertex {
            pos: top_right,
            color: colors[1],
            tex_pos: tex_top_right,
        });

        self.vertex_buffer.add_vertex(&Vertex {
            pos: bottom_right,
            color: colors[3],
            tex_pos: tex_bottom_right,
        });

        self.vertex_buffer.add_vertex(&Vertex {
            pos: bottom_left,
            color: colors[2],
            tex_pos: tex_bottom_left,
        });
    }

    pub fn update(&mut self) {
        self.vertex_buffer.upload();
    }

    /// Draws the `RectArray`.
    pub fn render(
        &self,
        graphics: &mut gfx::GraphicsContext,
        texture: Option<&gfx::Texture>,
        pos: gfx::FloatPos,
    ) {
        // to avoid artifacts
        let pos = gfx::FloatPos(pos.0 + 0.01, pos.1 + 0.01);

        // Safety: we are using the opengl functions correctly
        unsafe {
            let mut transform = graphics.renderer.normalization_transform.clone();

            transform.translate(pos);

            gl::UniformMatrix3fv(
                graphics.renderer.passthrough_shader.transform_matrix,
                1,
                gl::FALSE,
                transform.matrix.as_ptr(),
            );

            if let Some(texture) = texture {
                transform = texture.get_normalization_transform();
                gl::UniformMatrix3fv(
                    graphics
                        .renderer
                        .passthrough_shader
                        .texture_transform_matrix,
                    1,
                    gl::FALSE,
                    transform.matrix.as_ptr(),
                );
                gl::Uniform1i(graphics.renderer.passthrough_shader.has_texture, 1);
                gl::BindTexture(gl::TEXTURE_2D, texture.texture_handle);
            } else {
                gl::Uniform1i(graphics.renderer.passthrough_shader.has_texture, 0);
            }

            gl::Uniform4f(
                graphics.renderer.passthrough_shader.global_color,
                1.0,
                1.0,
                1.0,
                1.0,
            );

            self.vertex_buffer
                .draw(texture.is_some(), DrawMode::Triangles);
        }
    }
}
