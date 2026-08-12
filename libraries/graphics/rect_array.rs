use crate::libraries::graphics as gfx;

use super::draw_list::{DrawCommand, DrawTarget};
use super::vertex_buffer::{Vertex, VertexBuffer};

/// The struct `RectArray` is used to draw multiple rectangles with the same texture
/// and in one draw call. This is much faster than drawing each rectangle individually.
pub struct RectArray {
    vertex_buffer: VertexBuffer,
}

impl RectArray {
    /// Creates a new `RectArray`.
    #[must_use]
    pub const fn new() -> Self {
        Self { vertex_buffer: VertexBuffer::new() }
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

    /// Records a draw of the whole array, offset by `pos`.
    ///
    /// The command names the mesh by handle, so the array may be dropped or replaced before
    /// the frame is executed - which the world renderer does constantly, since a chunk that
    /// changes throws its whole `RectArray` away. `gpu_device` is what makes that safe.
    pub fn render(&self, target: &dyn DrawTarget, texture: Option<&gfx::Texture>, pos: gfx::FloatPos) {
        target.push_draw_command(DrawCommand::Mesh {
            mesh: self.vertex_buffer.get_handle(),
            texture: texture.map(|texture| (texture.get_handle(), texture.get_texture_size())),
            pos,
        });
    }
}
