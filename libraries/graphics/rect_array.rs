use crate::libraries::graphics as gfx;

use super::draw_list::{DrawCommand, DrawTarget};
use super::vertex_buffer::{Vertex, VertexBuffer};

/// Many rectangles sharing one texture, drawn in a single call. Much faster than drawing each
/// of them individually, which is why the world is built out of these.
pub struct RectArray {
    vertex_buffer: VertexBuffer,
}

impl RectArray {
    #[must_use]
    pub const fn new() -> Self {
        Self { vertex_buffer: VertexBuffer::new() }
    }

    /// Adds a rectangle as two triangles. `colors` is one colour per corner, in the order top
    /// left, top right, bottom left, bottom right.
    pub fn add_rect(&mut self, rect: &gfx::Rect, colors: &[gfx::Color; 4], tex_rect: &gfx::Rect) {
        let [top_left, top_right, bottom_left, bottom_right] = [
            (rect.pos, tex_rect.pos, colors[0]),
            (rect.pos + gfx::FloatSize(rect.size.0, 0.0), tex_rect.pos + gfx::FloatSize(tex_rect.size.0, 0.0), colors[1]),
            (rect.pos + gfx::FloatSize(0.0, rect.size.1), tex_rect.pos + gfx::FloatSize(0.0, tex_rect.size.1), colors[2]),
            (rect.pos + rect.size, tex_rect.pos + tex_rect.size, colors[3]),
        ];

        for (pos, tex_pos, color) in [top_left, top_right, bottom_left, top_right, bottom_right, bottom_left] {
            self.vertex_buffer.add_vertex(&Vertex { pos, color, tex_pos });
        }
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
