use crate::libraries::graphics as gfx;

use super::draw_list::{DrawCommand, DrawTarget, TextureHandle};
use super::gpu_garbage;
use super::{Color, Rect, Surface};

/// Texture is an image stored in gpu
pub struct Texture {
    handle: TextureHandle,
    size: gfx::FloatSize,
}

impl Texture {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            handle: TextureHandle::NONE,
            size: gfx::FloatSize(0.0, 0.0),
        }
    }

    /// Loads a Surface into gpu memory.
    #[must_use]
    pub fn load_from_surface(surface: &Surface) -> Self {
        let mut result = Self::new();
        result.size = gfx::FloatSize::from(surface.get_size());

        unsafe {
            gl::GenTextures(1, &raw mut result.handle.0);
            gl::BindTexture(gl::TEXTURE_2D, result.handle.0);

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

    /// A texture that reports a size but owns nothing on the GPU, for tests.
    ///
    /// Layout code only ever asks a texture for `get_texture_size`, so this is enough to
    /// drive a `Button` or a `Sprite` headlessly. It is safe to drop: the handle stays
    /// `TextureHandle::NONE`, so `free_texture` has nothing to park.
    #[cfg(test)]
    #[must_use]
    pub const fn new_sized(size: gfx::FloatSize) -> Self {
        Self { handle: TextureHandle::NONE, size }
    }

    /// Parks the OpenGL object for deletion if there is one.
    ///
    /// The name is handed to `gpu_garbage` rather than deleted outright, because a
    /// `DrawCommand` recorded earlier this frame may still name this texture. Textures are
    /// created and dropped inside `render_inner` all over the game, so this is the common
    /// case, not a corner one.
    fn free_texture(&mut self) {
        if self.handle != TextureHandle::NONE {
            gpu_garbage::delete_texture_later(self.handle.0);
            self.handle = TextureHandle::NONE;
            self.size = gfx::FloatSize(0.0, 0.0);
        }
    }

    #[must_use]
    pub const fn get_texture_size(&self) -> gfx::FloatSize {
        self.size
    }

    /// The backend's name for this texture, which is what a `DrawCommand` carries.
    #[must_use]
    pub const fn get_handle(&self) -> TextureHandle {
        self.handle
    }

    /// Records a draw of `src_rect` (the whole texture by default) at `pos`.
    pub fn render(&self, target: &dyn DrawTarget, scale: f32, pos: gfx::FloatPos, src_rect: Option<Rect>, flipped: bool, color: Option<Color>) {
        let src_rect = src_rect.unwrap_or_else(|| Rect::new(gfx::FloatPos(0.0, 0.0), self.get_texture_size()));

        // Also catches a texture that owns nothing on the GPU, since that reports a zero
        // size and so cannot produce a non-empty default source rectangle.
        if src_rect.size.0 <= 0.0 || src_rect.size.1 <= 0.0 {
            return;
        }

        target.push_draw_command(DrawCommand::Texture {
            texture: self.handle,
            texture_size: self.size,
            src_rect,
            pos,
            scale,
            flipped,
            color: color.unwrap_or(Color { r: 255, g: 255, b: 255, a: 255 }),
        });
    }
}

/// Free the surface when it goes out of scope.
impl Drop for Texture {
    fn drop(&mut self) {
        self.free_texture();
    }
}
