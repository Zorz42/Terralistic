use crate::vertex_buffer::{DrawMode, Vertex, VertexBuffer};
use crate::{Color, GraphicsContext, Rect, Texture};

/**
The struct RectArray is used to draw multiple rectangles with the same texture
and in one draw call. This is much faster than drawing each rectangle individually.
 */
pub struct RectArray {
    vertex_buffer: VertexBuffer,
}

impl RectArray {
    /**
    Creates a new RectArray.
     */
    pub fn new() -> Self {
        RectArray {
            vertex_buffer: VertexBuffer::new(),
        }
    }

    /**
    Adds a rectangle to the RectArray.
     */
    pub fn add_rect(&mut self, rect: &Rect, colors: &[Color; 4], tex_rect: &Rect) {
        let top_left = (rect.x as f32, rect.y as f32);
        let top_right = (rect.x as f32 + rect.w as f32, rect.y as f32);
        let bottom_left = (rect.x as f32, rect.y as f32 + rect.h as f32);
        let bottom_right = (rect.x as f32 + rect.w as f32, rect.y as f32 + rect.h as f32);

        let tex_top_left = (tex_rect.x as f32, tex_rect.y as f32);
        let tex_top_right = (tex_rect.x as f32 + tex_rect.w as f32, tex_rect.y as f32);
        let tex_bottom_left = (tex_rect.x as f32, tex_rect.y as f32 + tex_rect.h as f32);
        let tex_bottom_right = (
            tex_rect.x as f32 + tex_rect.w as f32,
            tex_rect.y as f32 + tex_rect.h as f32,
        );

        // first triangle
        self.vertex_buffer.add_vertex(&Vertex {
            x: top_left.0,
            y: top_left.1,
            color: colors[0],
            tex_x: tex_top_left.0,
            tex_y: tex_top_left.1,
        });

        self.vertex_buffer.add_vertex(&Vertex {
            x: top_right.0,
            y: top_right.1,
            color: colors[1],
            tex_x: tex_top_right.0,
            tex_y: tex_top_right.1,
        });

        self.vertex_buffer.add_vertex(&Vertex {
            x: bottom_left.0,
            y: bottom_left.1,
            color: colors[2],
            tex_x: tex_bottom_left.0,
            tex_y: tex_bottom_left.1,
        });

        // second triangle
        self.vertex_buffer.add_vertex(&Vertex {
            x: top_right.0,
            y: top_right.1,
            color: colors[1],
            tex_x: tex_top_right.0,
            tex_y: tex_top_right.1,
        });

        self.vertex_buffer.add_vertex(&Vertex {
            x: bottom_right.0,
            y: bottom_right.1,
            color: colors[3],
            tex_x: tex_bottom_right.0,
            tex_y: tex_bottom_right.1,
        });

        self.vertex_buffer.add_vertex(&Vertex {
            x: bottom_left.0,
            y: bottom_left.1,
            color: colors[2],
            tex_x: tex_bottom_left.0,
            tex_y: tex_bottom_left.1,
        });
    }

    pub fn update(&mut self) {
        self.vertex_buffer.upload();
    }

    /**
    Draws the RectArray.
     */
    pub fn render(
        &self, graphics: &mut GraphicsContext, texture: Option<&Texture>, x: i32, y: i32,
    ) {
        unsafe {
            let mut transform = graphics.renderer.normalization_transform.clone();

            transform.translate(x as f32, y as f32);

            gl::UniformMatrix3fv(
                graphics.renderer.passthrough_shader.transform_matrix,
                1,
                gl::FALSE,
                transform.matrix.as_ptr(),
            );

            if let Some(texture) = texture {
                let transform = texture.get_normalization_transform();
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
                .draw(!texture.is_none(), DrawMode::Triangles);
        }
    }
}
