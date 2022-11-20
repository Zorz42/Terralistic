use crate::color;

pub(crate) struct VertexImpl {
    pub(crate) x: f32,
    pub(crate) y: f32,
    pub(crate) color: color::Color,
    pub(crate) tex_x: f32,
    pub(crate) tex_y: f32,
}

pub(crate) struct VertexBufferImpl {
    vertices: Vec<f32>,
    indices: Vec<u32>,
    vertex_buffer: u32,
    index_buffer: u32,
    vertex_array: u32,
}

impl VertexBufferImpl {
    pub fn new() -> Self {
        let mut result = VertexBufferImpl{
            vertices: Vec::new(),
            indices: Vec::new(),
            vertex_buffer: 0,
            index_buffer: 0,
            vertex_array: 0,
        };
        unsafe {
            gl::GenBuffers(1, &mut result.vertex_buffer);
            gl::GenBuffers(1, &mut result.index_buffer);
            gl::GenVertexArrays(1, &mut result.vertex_array);
        }
        result
    }

    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &mut self.vertex_buffer);
            gl::DeleteBuffers(1, &mut self.index_buffer);
            gl::DeleteVertexArrays(1, &mut self.vertex_array);
        }
    }

    pub fn add_vertex(&mut self, vertex: &VertexImpl) {
        self.vertices.push(vertex.x);
        self.vertices.push(vertex.y);
        self.vertices.push(vertex.color.r as f32 / 255.0);
        self.vertices.push(vertex.color.g as f32 / 255.0);
        self.vertices.push(vertex.color.b as f32 / 255.0);
        self.vertices.push(vertex.color.a as f32 / 255.0);
        self.vertices.push(vertex.tex_x);
        self.vertices.push(vertex.tex_y);

        self.indices.push(self.indices.len() as u32);
    }

    pub fn clear(&mut self) {
        self.vertices.clear();
        self.indices.clear();
    }

    pub fn upload(&self) {
        unsafe {
            gl::BindVertexArray(self.vertex_array);

            gl::BindBuffer(gl::ARRAY_BUFFER, self.vertex_buffer);
            gl::BufferData(gl::ARRAY_BUFFER, (self.vertices.len() * 4) as isize, self.vertices.as_ptr() as *const _, gl::STATIC_DRAW);

            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.index_buffer);
            gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, (self.indices.len() * 4) as isize, self.indices.as_ptr() as *const _, gl::STATIC_DRAW);
        }
    }

    pub fn draw(&self, has_texture: bool) {
        unsafe {
            gl::BindVertexArray(self.vertex_array);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vertex_buffer);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.index_buffer);

            gl::VertexAttribPointer(0, 2, gl::FLOAT, gl::FALSE, (8 * 4) as i32, std::ptr::null());
            gl::VertexAttribPointer(1, 4, gl::FLOAT, gl::FALSE, (8 * 4) as i32, (2 * 4) as *const _);
            if has_texture {
                gl::VertexAttribPointer(2, 2, gl::FLOAT, gl::FALSE, (8 * 4) as i32, (6 * 4) as *const _);
            }

            gl::EnableVertexAttribArray(0);
            gl::EnableVertexAttribArray(1);
            if has_texture {
                gl::EnableVertexAttribArray(2);
            }
            gl::DrawElements(gl::TRIANGLES, self.indices.len() as i32, gl::UNSIGNED_INT, std::ptr::null());
            gl::DisableVertexAttribArray(0);
            gl::DisableVertexAttribArray(1);
            if has_texture {
                gl::DisableVertexAttribArray(2);
            }
        }
    }
}