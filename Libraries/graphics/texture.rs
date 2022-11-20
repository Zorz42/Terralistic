use crate::surface;
use crate::transformation;
use crate::rect_shape;
use crate::color;
use crate::renderer;

/*
Texture is an image stored in gpu
*/
pub struct Texture {
    pub(crate) texture_handle: u32,
    pub(crate) transform: transformation::Transformation,
    width: i32,
    height: i32,
}

impl Texture {
    pub fn new() -> Self {
        Texture{
            texture_handle: u32::MAX,
            transform: transformation::Transformation::new(),
            width: 0,
            height: 0,
        }
    }
    /*
    Deletes the current texture if it exists.
    */
    fn free_texture(&mut self) {
        if self.texture_handle != u32::MAX {
            unsafe {
                gl::DeleteTextures(1, &self.texture_handle);
            }
            self.texture_handle = u32::MAX;
            self.width = 0;
            self.height = 0;
        }
    }

    pub fn get_texture_width(&self) -> i32 {
        self.width
    }

    pub fn get_texture_height(&self) -> i32 {
        self.height
    }

    /*
    Loads a Surface into gpu memory.
    */
    pub fn load_from_surface(&mut self, surface: &surface::Surface) {
        self.free_texture();

        self.width = surface.get_width();
        self.height = surface.get_height();

        unsafe {
            gl::GenTextures(1, &mut self.texture_handle);
            gl::BindTexture(gl::TEXTURE_2D, self.texture_handle); // all upcoming GL_TEXTURE_2D operations now have effect on this texture object
            // set the texture wrapping parameters
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32); // set texture wrapping to gl::REPEAT (default wrapping method)
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
            // set texture filtering parameters
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
            // load image, create texture and generate mipmaps
            let data = surface.pixels.clone();
            gl::TexImage2D(gl::TEXTURE_2D,
                           0,
                           gl::RGB as i32,
                           self.width as i32,
                           self.height as i32,
                           0,
                           gl::BGRA,
                           gl::UNSIGNED_BYTE,
                           &data[0] as *const u8 as *const std::ffi::c_void);
            gl::GenerateMipmap(gl::TEXTURE_2D);
        }
    }

    pub fn render(&self, renderer: &renderer::Renderer, scale: f32, x: i32, y: i32, src_rect: rect_shape::RectShape, flipped: bool, color: color::Color) {
        unsafe {
            let vertices: [f32; 32] = [
                // positions // colors             // texture coords
                1.0, 0.0,    1.0, 1.0, 1.0, 1.0,   1.0, 0.0, // top right
                1.0, 1.0,    1.0, 1.0, 1.0, 1.0,   1.0, 1.0, // bottom right
                0.0, 1.0,    1.0, 1.0, 1.0, 1.0,   0.0, 1.0, // bottom left
                0.0, 0.0,    1.0, 1.0, 1.0, 1.0,   0.0, 0.0, // top left
            ];
            let indices = [
                0, 1, 3,  // first Triangle
                1, 2, 3,  // second Triangle
            ];

            let (mut VBO, mut VAO, mut EBO) = (0, 0, 0);
            gl::GenVertexArrays(1, &mut VAO);
            gl::GenBuffers(1, &mut VBO);
            gl::GenBuffers(1, &mut EBO);

            gl::BindVertexArray(VAO);

            gl::BindBuffer(gl::ARRAY_BUFFER, VBO);
            gl::BufferData(gl::ARRAY_BUFFER,
                           (vertices.len() * std::mem::size_of::<gl::types::GLfloat>()) as gl::types::GLsizeiptr,
                           &vertices[0] as *const f32 as *const std::ffi::c_void,
                           gl::STATIC_DRAW);

            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, EBO);
            gl::BufferData(gl::ELEMENT_ARRAY_BUFFER,
                           (indices.len() * std::mem::size_of::<gl::types::GLfloat>()) as gl::types::GLsizeiptr,
                           &indices[0] as *const i32 as *const std::ffi::c_void,
                           gl::STATIC_DRAW);

            let stride = 8 * std::mem::size_of::<gl::types::GLfloat>() as gl::types::GLsizei;
            // position attribute
            gl::VertexAttribPointer(0, 2, gl::FLOAT, gl::FALSE, stride, std::ptr::null());
            gl::EnableVertexAttribArray(0);
            // color attribute
            gl::VertexAttribPointer(1, 4, gl::FLOAT, gl::FALSE, stride, (2 * std::mem::size_of::<gl::types::GLfloat>()) as *const std::ffi::c_void);
            gl::EnableVertexAttribArray(1);
            // texture coord attribute
            gl::VertexAttribPointer(2, 2, gl::FLOAT, gl::FALSE, stride, (6 * std::mem::size_of::<gl::types::GLfloat>()) as *const std::ffi::c_void);
            gl::EnableVertexAttribArray(2);

            let mut transform = renderer.normalization_transform.clone();

            if flipped {
                transform.translate(src_rect.w as f32 * scale + x as f32 * 2.0, 0.0);
                transform.stretch(-1.0, 1.0);
            }

            transform.translate(x as f32, y as f32);
            transform.stretch(src_rect.w as f32 * scale, src_rect.h as f32 * scale);

            gl::UniformMatrix3fv(renderer.uniforms.transform_matrix, 1, gl::FALSE, transform.matrix.as_ptr());

            let mut transform = transformation::Transformation::new();
            transform.translate(-1.0, 0.0);
            transform.stretch(1.0 / self.get_texture_width() as f32, 1.0 / self.get_texture_height() as f32);
            transform.translate(src_rect.x as f32, src_rect.y as f32);
            transform.stretch(src_rect.w as f32, src_rect.h as f32);

            gl::UniformMatrix3fv(renderer.uniforms.texture_transform_matrix, 1, gl::FALSE, transform.matrix.as_ptr());

            gl::Uniform4f(renderer.uniforms.global_color, 1.0, 1.0, 1.0, 1.0);

            gl::Uniform1i(renderer.uniforms.has_texture, 1);

            gl::BindVertexArray(VAO);
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null());

        }
    }
}