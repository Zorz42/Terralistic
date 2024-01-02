use crate::libraries::graphics as gfx;

use super::renderer::GraphicsContext;
use super::transformation::Transformation;
use super::vertex_buffer::DrawMode;
use super::{Color, Rect, Surface};

/// Texture is an image stored in gpu
pub struct Texture {
    pub(super) texture_handle: u32,
    size: gfx::FloatSize,
}

impl Texture {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            texture_handle: u32::MAX,
            size: gfx::FloatSize(0.0, 0.0),
        }
    }

    /// Loads a Surface into gpu memory.
    #[must_use]
    pub fn load_from_surface(surface: &Surface) -> Self {
        let mut result = Self::new();
        result.size = gfx::FloatSize::from(surface.get_size());

        unsafe {
            gl::GenTextures(1, &mut result.texture_handle);
            gl::BindTexture(gl::TEXTURE_2D, result.texture_handle);

            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);

            // set the texture wrapping parameters
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);

            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA as i32,
                result.size.0 as i32,
                result.size.1 as i32,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                surface.pixels.as_ptr() as *const std::ffi::c_void,
            );
            //gl::GenerateMipmap(gl::TEXTURE_2D);
        }

        result
    }

    /// Deletes the current texture if it exists.
    fn free_texture(&mut self) {
        if self.texture_handle != u32::MAX {
            unsafe {
                gl::DeleteTextures(1, &self.texture_handle);
            }
            self.texture_handle = u32::MAX;
            self.size = gfx::FloatSize(0.0, 0.0);
        }
    }

    #[must_use]
    pub const fn get_texture_size(&self) -> gfx::FloatSize {
        self.size
    }

    pub(super) fn get_normalization_transform(&self) -> Transformation {
        let mut result = Transformation::new();
        result.stretch((1.0 / self.size.0, 1.0 / self.size.1));
        result
    }

    pub fn render(&self, graphics: &GraphicsContext, scale: f32, pos: gfx::FloatPos, src_rect: Option<Rect>, flipped: bool, color: Option<Color>) {
        let src_rect = src_rect.unwrap_or_else(|| Rect::new(gfx::FloatPos(0.0, 0.0), self.get_texture_size()));

        if src_rect.size.0 <= 0.0 || src_rect.size.1 <= 0.0 {
            return;
        }

        let color = color.unwrap_or(Color { r: 255, g: 255, b: 255, a: 255 });

        let mut transform = graphics.normalization_transform.clone();

        if flipped {
            transform.translate(gfx::FloatPos(src_rect.size.0 * scale + pos.0 * 2.0, 0.0));
            transform.stretch((-1.0, 1.0));
        }

        transform.translate(pos);
        transform.stretch((src_rect.size.0 * scale, src_rect.size.1 * scale));

        unsafe {
            gl::UniformMatrix3fv(graphics.passthrough_shader.transform_matrix, 1, gl::FALSE, transform.matrix.as_ptr());

            transform = self.get_normalization_transform();
            transform.translate(src_rect.pos);
            transform.stretch((src_rect.size.0 + 0.1, src_rect.size.1 + 0.1));

            gl::UniformMatrix3fv(graphics.passthrough_shader.texture_transform_matrix, 1, gl::FALSE, transform.matrix.as_ptr());
            gl::Uniform4f(
                graphics.passthrough_shader.global_color,
                color.r as f32 / 255.0,
                color.g as f32 / 255.0,
                color.b as f32 / 255.0,
                color.a as f32 / 255.0,
            );
            gl::Uniform1i(graphics.passthrough_shader.has_texture, 1);

            gl::BindTexture(gl::TEXTURE_2D, self.texture_handle);

            graphics.passthrough_shader.rect_vertex_buffer.draw(true, DrawMode::Triangles);
        }
    }
}

/// Free the surface when it goes out of scope.
impl Drop for Texture {
    fn drop(&mut self) {
        self.free_texture();
    }
}
