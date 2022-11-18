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
            gl::BindTexture(gl::TEXTURE_2D, self.texture_handle);
            gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGBA as i32, self.width, self.height, 0, gl::BGRA, gl::UNSIGNED_BYTE, &surface.pixels[0] as *const u8 as *const std::os::raw::c_void);

            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
        }

        self.transform = transformation::Transformation::new();
        self.transform.translate(0.0, 1.0);
        self.transform.stretch(1.0 / self.width as f32, -1.0 / self.height as f32);
    }

    pub fn render(&self, renderer: &renderer::Renderer, scale: f32, x: i32, y: i32, src_rect: rect_shape::RectShape, flipped: bool, color: color::Color) {
        if scale <= 0.0 {
            panic!("Texture scale must be positive.");
        }

        let mut transform = self.transform.clone();
        transform.translate(src_rect.x as f32, src_rect.y as f32);
        transform.stretch(src_rect.w as f32, src_rect.h as f32);

        unsafe {
            gl::UniformMatrix3fv(renderer.uniforms.texture_transform_matrix, 1, gl::FALSE, &transform.matrix[0]);

            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, self.texture_handle);

            gl::Uniform1i(renderer.uniforms.texture_sampler, 0);
            gl::Uniform1i(renderer.uniforms.has_texture, 1);
            gl::Uniform1i(renderer.uniforms.has_color_buffer, 0);
        }

        let mut transform = renderer.normalization_transform.clone();

        if flipped  {
            transform.translate(src_rect.w as f32 * scale + x as f32 * 2.0, 0.0);
            transform.stretch(-1.0, 1.0);
        }

        transform.translate(x as f32, y as f32);
        transform.stretch(src_rect.w as f32 * scale, src_rect.h as f32 * scale);

        unsafe {
            gl::UniformMatrix3fv(renderer.uniforms.transform_matrix, 1, gl::FALSE, &transform.matrix[0]);
            gl::Uniform4f(renderer.uniforms.default_color, color.r as f32 / 255.0, color.g as f32 / 255.0, color.b as f32 / 255.0, color.a as f32 / 255.0);

            gl::EnableVertexAttribArray(renderer::SHADER_TEXTURE_COORD_BUFFER);

            gl::BindBuffer(gl::ARRAY_BUFFER, renderer.rect_vertex_buffer);
            gl::VertexAttribPointer(renderer::SHADER_VERTEX_BUFFER, 2, gl::FLOAT, gl::FALSE, 0, std::ptr::null());

            gl::BindBuffer(gl::ARRAY_BUFFER, renderer.rect_vertex_buffer);
            gl::VertexAttribPointer(renderer::SHADER_TEXTURE_COORD_BUFFER, 2, gl::FLOAT, gl::FALSE, 0, std::ptr::null());

            gl::DrawArrays(gl::TRIANGLES, 0, 6);

            gl::DisableVertexAttribArray(renderer::SHADER_TEXTURE_COORD_BUFFER);
        }
    }
}