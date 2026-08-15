use crate::libraries::graphics as gfx;

use super::color;
use super::draw_list::MeshHandle;
use super::gpu_device;

/// The id of a buffer that has never been uploaded. Zero is never handed out by the registry.
const NO_MESH: u32 = 0;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Vertex {
    pub(super) pos: gfx::FloatPos,
    pub(super) color: color::Color,
    pub(super) tex_pos: gfx::FloatPos,
}

/// Vertices on their way to the GPU, and the buffer's id once they get there. `upload` creates
/// the resource, so a buffer that is filled and never uploaded costs only its `Vec` - and one
/// that is uploaded gives the `Vec` back, the GPU having the data by then.
pub struct VertexBuffer {
    /// Staged vertices, emptied by `upload`.
    vertices: Vec<f32>,
    id: u32,
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
        Self { vertices: Vec::new(), id: NO_MESH }
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

    /// Sends the staged vertices to the GPU and drops the CPU copy.
    ///
    /// **Taking the vertices rather than borrowing is the point**: a mesh is built once and
    /// then only drawn, so keeping them is a second copy of every chunk mesh in RAM - ~50 KB
    /// each across three caches of a thousand.
    ///
    /// With nothing staged there is nothing to send, so an already uploaded mesh is left alone
    /// rather than emptied. With no device the vertices stay staged: not having one is a
    /// supported state, and throwing the data away would make it a failure.
    pub fn upload(&mut self) {
        let Some(gpu) = gpu_device::get() else { return };
        if self.vertices.is_empty() {
            return;
        }
        let vertices = std::mem::take(&mut self.vertices);

        if self.id != NO_MESH {
            gpu_device::delete_mesh_later(self.id);
        }
        self.id = gpu.create_mesh(&vertices);
    }

    /// The backend's name for this mesh, which is what a `DrawCommand` carries. It may outlive
    /// the buffer: the resource is parked on drop and released after the frame runs.
    pub(super) const fn get_handle(&self) -> MeshHandle {
        MeshHandle(self.id)
    }
}
