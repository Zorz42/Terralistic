use crate::libraries::graphics as gfx;

use super::color;
use super::draw_list::MeshHandle;
use super::gpu_device;
use super::wgpu_backend::VERTEX_FLOATS;

/// The id of a buffer that has never been uploaded. Zero is never handed out by the registry.
const NO_MESH: u32 = 0;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Vertex {
    pub(super) pos: gfx::FloatPos,
    pub(super) color: color::Color,
    pub(super) tex_pos: gfx::FloatPos,
}

/// Vertices on their way to the GPU, and the id of the buffer once they get there.
///
/// `upload` is what creates the GPU resource, so a buffer that is filled and never uploaded
/// costs nothing but the `Vec`.
pub struct VertexBuffer {
    vertices: Vec<f32>,
    id: u32,
    vertex_count: u32,
}

/// Parks the GPU resource rather than releasing it, because a `DrawCommand` recorded earlier
/// this frame may still name it - see `gpu_device`.
impl Drop for VertexBuffer {
    fn drop(&mut self) {
        if self.id != NO_MESH {
            gpu_device::delete_mesh_later(self.id);
        }
    }
}

impl VertexBuffer {
    pub const fn new() -> Self {
        Self {
            vertices: Vec::new(),
            id: NO_MESH,
            vertex_count: 0,
        }
    }

    pub fn add_vertex(&mut self, vertex: &Vertex) {
        self.vertices.extend_from_slice(&[
            vertex.pos.0,
            vertex.pos.1,
            vertex.color.r as f32 / 255.0,
            vertex.color.g as f32 / 255.0,
            vertex.color.b as f32 / 255.0,
            vertex.color.a as f32 / 255.0,
            vertex.tex_pos.0,
            vertex.tex_pos.1,
        ]);
    }

    /// Sends the vertices to the GPU. Without a device this does nothing, and the mesh draws
    /// nothing.
    pub fn upload(&mut self) {
        self.vertex_count = (self.vertices.len() / VERTEX_FLOATS) as u32;
        let Some(gpu) = gpu_device::get() else { return };

        if self.id != NO_MESH {
            gpu_device::delete_mesh_later(self.id);
        }
        self.id = gpu.create_mesh(&self.vertices, self.vertex_count);
    }

    /// The backend's name for this mesh, which is what a `DrawCommand` carries.
    ///
    /// A handle may outlive the buffer it names: the GPU resource is parked on drop and only
    /// released once the frame's commands have run.
    pub(super) const fn get_handle(&self) -> MeshHandle {
        MeshHandle {
            id: self.id,
            vertex_count: self.vertex_count,
        }
    }
}
