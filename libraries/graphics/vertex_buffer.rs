use crate::libraries::graphics as gfx;

use super::color;
use super::draw_list::MeshHandle;
use super::gpu_garbage;

#[derive(Debug, Clone, Copy)]
pub enum DrawMode {
    Triangles,
    Lines,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Vertex {
    pub(super) pos: gfx::FloatPos,
    pub(super) color: color::Color,
    pub(super) tex_pos: gfx::FloatPos,
}

pub struct VertexBuffer {
    vertices: Vec<f32>,
    indices: Vec<u32>,
    vertex_buffer: u32,
    index_buffer: u32,
    vertex_array: u32,
}

/// Parks the OpenGL objects rather than deleting them, because a `DrawCommand` recorded
/// earlier this frame may still refer to them - see `gpu_garbage`.
impl Drop for VertexBuffer {
    fn drop(&mut self) {
        gpu_garbage::delete_buffer_later(self.vertex_buffer);
        gpu_garbage::delete_buffer_later(self.index_buffer);
        gpu_garbage::delete_vertex_array_later(self.vertex_array);
    }
}

impl VertexBuffer {
    pub fn new() -> Self {
        let mut result = Self {
            vertices: Vec::new(),
            indices: Vec::new(),
            vertex_buffer: 0,
            index_buffer: 0,
            vertex_array: 0,
        };
        unsafe {
            gl::GenBuffers(1, &raw mut result.vertex_buffer);
            gl::GenBuffers(1, &raw mut result.index_buffer);
            gl::GenVertexArrays(1, &raw mut result.vertex_array);
        }
        result
    }

    pub fn add_vertex(&mut self, vertex: &Vertex) {
        let index = self.vertices.len() as u32 / 8;
        self.indices.push(index);

        self.vertices.push(vertex.pos.0);
        self.vertices.push(vertex.pos.1);
        self.vertices.push(vertex.color.r as f32 / 255.0);
        self.vertices.push(vertex.color.g as f32 / 255.0);
        self.vertices.push(vertex.color.b as f32 / 255.0);
        self.vertices.push(vertex.color.a as f32 / 255.0);
        self.vertices.push(vertex.tex_pos.0);
        self.vertices.push(vertex.tex_pos.1);
    }

    pub fn upload(&self) {
        unsafe {
            gl::BindVertexArray(self.vertex_array);

            gl::BindBuffer(gl::ARRAY_BUFFER, self.vertex_buffer);
            gl::BufferData(gl::ARRAY_BUFFER, (self.vertices.len() * 4) as isize, self.vertices.as_ptr().cast(), gl::STATIC_DRAW);

            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.index_buffer);
            gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, (self.indices.len() * 4) as isize, self.indices.as_ptr().cast(), gl::STATIC_DRAW);
        }
    }

    /// The backend's name for this mesh, which is what a `DrawCommand` carries.
    ///
    /// A handle can outlive the buffer it names. That is deliberate and safe: the OpenGL
    /// objects are parked in `gpu_garbage` on drop and only released once the frame's
    /// commands have run.
    pub(super) const fn get_handle(&self) -> MeshHandle {
        MeshHandle {
            vertex_array: self.vertex_array,
            vertex_buffer: self.vertex_buffer,
            index_buffer: self.index_buffer,
            index_count: self.indices.len() as u32,
        }
    }

    /// Draws this buffer immediately. Only the backend's own built-in rectangle meshes use
    /// this; everything else goes through a `DrawCommand::Mesh` and `draw_mesh`.
    pub(super) fn draw(&self, has_texture: bool, mode: DrawMode) {
        draw_mesh(self.get_handle(), has_texture, mode);
    }
}

/// Issues the draw for an already uploaded mesh.
pub(super) fn draw_mesh(mesh: MeshHandle, has_texture: bool, mode: DrawMode) {
    unsafe {
        gl::BindVertexArray(mesh.vertex_array);
        gl::BindBuffer(gl::ARRAY_BUFFER, mesh.vertex_buffer);
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, mesh.index_buffer);

        gl::VertexAttribPointer(0, 2, gl::FLOAT, gl::FALSE, 8 * 4, std::ptr::null());
        gl::VertexAttribPointer(1, 4, gl::FLOAT, gl::FALSE, 8 * 4, (2 * 4) as *const _);
        if has_texture {
            gl::VertexAttribPointer(2, 2, gl::FLOAT, gl::FALSE, 8 * 4, (6 * 4) as *const _);
        }

        gl::EnableVertexAttribArray(0);
        gl::EnableVertexAttribArray(1);
        if has_texture {
            gl::EnableVertexAttribArray(2);
        }

        let gl_mode = match mode {
            DrawMode::Triangles => gl::TRIANGLES,
            DrawMode::Lines => gl::LINES,
        };

        gl::DrawElements(gl_mode, mesh.index_count as i32, gl::UNSIGNED_INT, std::ptr::null());

        gl::DisableVertexAttribArray(0);
        gl::DisableVertexAttribArray(1);
        if has_texture {
            gl::DisableVertexAttribArray(2);
        }
    }
}
