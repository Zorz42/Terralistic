use super::color;
use crate::libraries::graphics as gfx;
use std::collections::HashMap;

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
    indices_map: HashMap<Vertex, u32>,
    vertex_buffer: u32,
    index_buffer: u32,
    vertex_array: u32,
}

impl Drop for VertexBuffer {
    fn drop(&mut self) {
        // Safety: We are deleting the buffers and vertex array. They are either 0 or valid.
        unsafe {
            gl::DeleteBuffers(1, &self.vertex_buffer);
            gl::DeleteBuffers(1, &self.index_buffer);
            gl::DeleteVertexArrays(1, &self.vertex_array);
        }
    }
}

impl VertexBuffer {
    pub fn new() -> Self {
        let mut result = Self {
            vertices: Vec::new(),
            indices: Vec::new(),
            indices_map: HashMap::new(),
            vertex_buffer: 0,
            index_buffer: 0,
            vertex_array: 0,
        };
        // Safety: We are generating the buffers and vertex array.
        unsafe {
            gl::GenBuffers(1, &mut result.vertex_buffer);
            gl::GenBuffers(1, &mut result.index_buffer);
            gl::GenVertexArrays(1, &mut result.vertex_array);
        }
        result
    }

    pub fn add_vertex(&mut self, vertex: &Vertex) {
        if let Some(index) = self.indices_map.get(vertex) {
            self.indices.push(*index);
            return;
        }

        let index = self.vertices.len() as u32 / 8;
        self.indices.push(index);
        self.indices_map.insert(*vertex, index);

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
        // Safety: all handles are valid, because they are either 0 or generated in the constructor.
        unsafe {
            gl::BindVertexArray(self.vertex_array);

            gl::BindBuffer(gl::ARRAY_BUFFER, self.vertex_buffer);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (self.vertices.len() * 4) as isize,
                self.vertices.as_ptr().cast(),
                gl::STATIC_DRAW,
            );

            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.index_buffer);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (self.indices.len() * 4) as isize,
                self.indices.as_ptr().cast(),
                gl::STATIC_DRAW,
            );
        }
    }

    pub fn draw(&self, has_texture: bool, mode: DrawMode) {
        // Safety: all handles are valid, because they are either 0 or generated in the constructor.
        unsafe {
            gl::BindVertexArray(self.vertex_array);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vertex_buffer);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.index_buffer);

            gl::VertexAttribPointer(0, 2, gl::FLOAT, gl::FALSE, 8 * 4, core::ptr::null());
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

            gl::DrawElements(
                gl_mode,
                self.indices.len() as i32,
                gl::UNSIGNED_INT,
                core::ptr::null(),
            );

            gl::DisableVertexAttribArray(0);
            gl::DisableVertexAttribArray(1);
            if has_texture {
                gl::DisableVertexAttribArray(2);
            }
        }
    }
}
