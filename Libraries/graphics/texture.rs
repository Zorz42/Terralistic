use std::ffi::c_void;
use crate::{Color, Rect, surface, Surface};
use crate::transformation;
use crate::rect;
use crate::color;
use crate::renderer;
use crate::renderer::Renderer;
use crate::transformation::Transformation;

/*
Texture is an image stored in gpu
*/
pub struct Texture {
    pub(crate) texture_handle: u32,
    width: i32,
    height: i32,
}

impl Texture {
    pub fn new() -> Self {
        Texture{
            texture_handle: u32::MAX,
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
    pub fn load_from_surface(&mut self, surface: &Surface) {
        self.free_texture();

        self.width = surface.get_width();
        self.height = surface.get_height();

        unsafe {
            gl::GenTextures(1, &mut self.texture_handle);
            gl::BindTexture(gl::TEXTURE_2D, self.texture_handle);

            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);

            let data = surface.pixels.clone();
            gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGB as i32, self.width as i32, self.height as i32, 0, gl::RGBA, gl::UNSIGNED_BYTE, &data[0] as *const u8 as *const c_void);
            gl::GenerateMipmap(gl::TEXTURE_2D);
        }
    }

    pub fn render(&self, renderer: &Renderer, scale: f32, x: i32, y: i32, src_rect: Rect, flipped: bool, color: Color) {
        unsafe {
            let mut transform = renderer.normalization_transform.clone();

            if flipped {
                transform.translate(src_rect.w as f32 * scale + x as f32 * 2.0, 0.0);
                transform.stretch(-1.0, 1.0);
            }

            transform.translate(x as f32, y as f32);
            transform.stretch(src_rect.w as f32 * scale, src_rect.h as f32 * scale);

            gl::UniformMatrix3fv(renderer.uniforms.transform_matrix, 1, gl::FALSE, transform.matrix.as_ptr());

            let mut transform = Transformation::new();
            transform.translate(-1.0, 0.0);
            transform.stretch(1.0 / self.get_texture_width() as f32, 1.0 / self.get_texture_height() as f32);
            transform.translate(src_rect.x as f32, src_rect.y as f32);
            transform.stretch(src_rect.w as f32, src_rect.h as f32);

            gl::UniformMatrix3fv(renderer.uniforms.texture_transform_matrix, 1, gl::FALSE, transform.matrix.as_ptr());
            gl::Uniform4f(renderer.uniforms.global_color, color.r as f32 / 255.0, color.g as f32 / 255.0, color.b as f32 / 255.0, color.a as f32 / 255.0);
            gl::Uniform1i(renderer.uniforms.has_texture, 1);

            gl::BindTexture(gl::TEXTURE_2D, self.texture_handle);

            renderer.rect_vertex_buffer.draw(true);
        }
    }
}