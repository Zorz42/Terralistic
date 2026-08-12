use crate::libraries::graphics as gfx;

use super::color;
use super::draw_list::MeshHandle;
use super::gpu_device;
use super::wgpu_backend::VERTEX_FLOATS;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Vertex {
    pub(super) pos: gfx::FloatPos,
    pub(super) color: color::Color,
    pub(super) tex_pos: gfx::FloatPos,
}

/// Vertices on their way to the GPU, and the id of the buffer once they get there.
///
/// `upload` is what actually creates the GPU resource, so a buffer that is filled and never
/// uploaded costs nothing but the `Vec`.
pub struct VertexBuffer {
    vertices: Vec<f32>,
    /// Registry id, or `MeshHandle::NONE`'s id while nothing has been uploaded.
    id: u32,
    vertex_count: u32,
}

/// Parks the GPU resource rather than releasing it, because a `DrawCommand` recorded earlier
/// this frame may still name it - see `gpu_device`.
impl Drop for VertexBuffer {
    fn drop(&mut self) {
        if self.id != 0 {
            gpu_device::delete_mesh_later(self.id);
        }
    }
}

impl VertexBuffer {
    pub const fn new() -> Self {
        Self {
            vertices: Vec::new(),
            id: 0,
            vertex_count: 0,
        }
    }

    pub fn add_vertex(&mut self, vertex: &Vertex) {
        self.vertices.push(vertex.pos.0);
        self.vertices.push(vertex.pos.1);
        self.vertices.push(vertex.color.r as f32 / 255.0);
        self.vertices.push(vertex.color.g as f32 / 255.0);
        self.vertices.push(vertex.color.b as f32 / 255.0);
        self.vertices.push(vertex.color.a as f32 / 255.0);
        self.vertices.push(vertex.tex_pos.0);
        self.vertices.push(vertex.tex_pos.1);
    }

    /// Sends the vertices to the GPU. Without a device this does nothing, and the mesh draws
    /// nothing.
    pub fn upload(&mut self) {
        let count = (self.vertices.len() / VERTEX_FLOATS) as u32;
        let Some(gpu) = gpu_device::get() else {
            self.vertex_count = count;
            return;
        };

        if self.id != 0 {
            gpu_device::delete_mesh_later(self.id);
        }
        self.id = gpu.create_mesh(&self.vertices, count);
        self.vertex_count = count;
    }

    /// The backend's name for this mesh, which is what a `DrawCommand` carries.
    ///
    /// A handle can outlive the buffer it names. That is deliberate and safe: the GPU
    /// resource is parked on drop and only released once the frame's commands have run.
    pub(super) const fn get_handle(&self) -> MeshHandle {
        MeshHandle {
            id: self.id,
            vertex_count: self.vertex_count,
        }
    }
}
