use super::renderer::Renderer;
use super::transformation::Transformation;
use super::vertex_buffer::DrawMode;
use super::{Color, Rect, Surface};
use std::ffi::c_void;

/**
Texture is an image stored in gpu
 */
pub struct Texture {
    pub(super) texture_handle: u32,
    width: i32,
    height: i32,
}

impl Default for Texture {
    fn default() -> Self {
        Self::new()
    }
}

impl Texture {
    #[must_use]
    pub fn new() -> Self {
        Self {
            texture_handle: u32::MAX,
            width: 0,
            height: 0,
        }
    }

    /**
    Loads a Surface into gpu memory.
     */
    #[must_use]
    pub fn load_from_surface(surface: &Surface) -> Self {
        let mut result = Self::new();
        result.width = surface.get_width();
        result.height = surface.get_height();

        unsafe {
            gl::GenTextures(1, &mut result.texture_handle);
            gl::BindTexture(gl::TEXTURE_2D, result.texture_handle);

            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);

            let data = surface.pixels.clone();
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA as i32,
                result.width,
                result.height,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                std::ptr::addr_of!(data[0]) as *const c_void,
            );
            gl::GenerateMipmap(gl::TEXTURE_2D);
        }

        result
    }

    /**
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

    #[must_use]
    pub fn get_texture_width(&self) -> i32 {
        self.width
    }

    #[must_use]
    pub fn get_texture_height(&self) -> i32 {
        self.height
    }

    pub(super) fn get_normalization_transform(&self) -> Transformation {
        let mut result = Transformation::new();
        result.translate(-1.0, 1.0);
        result.stretch(1.0 / self.width as f32, 1.0 / self.height as f32);
        result
    }

    pub fn render(
        &self,
        renderer: &Renderer,
        scale: f32,
        pos: (i32, i32),
        src_rect: Option<Rect>,
        flipped: bool,
        color: Option<Color>,
    ) {
        let src_rect = src_rect.unwrap_or(Rect::new(
            0,
            0,
            self.get_texture_width(),
            self.get_texture_height(),
        ));
        let color = color.unwrap_or(Color {
            r: 255,
            g: 255,
            b: 255,
            a: 255,
        });

        unsafe {
            let mut transform = renderer.normalization_transform.clone();

            if flipped {
                transform.translate(src_rect.w as f32 * scale + pos.0 as f32 * 2.0, 0.0);
                transform.stretch(-1.0, 1.0);
            }

            transform.translate(pos.0 as f32, pos.1 as f32);
            transform.stretch(src_rect.w as f32 * scale, src_rect.h as f32 * scale);

            gl::UniformMatrix3fv(
                renderer.passthrough_shader.transform_matrix,
                1,
                gl::FALSE,
                transform.matrix.as_ptr(),
            );

            let mut transform = self.get_normalization_transform();
            transform.translate(src_rect.x as f32, src_rect.y as f32);
            transform.stretch(src_rect.w as f32, src_rect.h as f32);

            gl::UniformMatrix3fv(
                renderer.passthrough_shader.texture_transform_matrix,
                1,
                gl::FALSE,
                transform.matrix.as_ptr(),
            );
            gl::Uniform4f(
                renderer.passthrough_shader.global_color,
                color.r as f32 / 255.0,
                color.g as f32 / 255.0,
                color.b as f32 / 255.0,
                color.a as f32 / 255.0,
            );
            gl::Uniform1i(renderer.passthrough_shader.has_texture, 1);

            gl::BindTexture(gl::TEXTURE_2D, self.texture_handle);

            renderer
                .passthrough_shader
                .rect_vertex_buffer
                .draw(true, DrawMode::Triangles);
        }
    }
}

/**
Free the surface when it goes out of scope.
 */
impl Drop for Texture {
    fn drop(&mut self) {
        self.free_texture();
    }
}
