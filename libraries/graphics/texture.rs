use crate::libraries::graphics as gfx;

use super::draw_list::{DrawCommand, DrawTarget, TextureHandle};
use super::gpu_device;
use super::{Color, Rect, Surface};

/// An image stored on the GPU.
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

    /// Uploads a surface to the GPU.
    ///
    /// With no device - a process that never opened a window, such as `cargo test` - this
    /// returns a texture that knows its size but owns nothing, so layout code still works and
    /// drawing it is a no-op.
    #[must_use]
    pub fn load_from_surface(surface: &Surface) -> Self {
        Self {
            handle: gpu_device::get().map_or(TextureHandle::NONE, |gpu| TextureHandle(gpu.create_texture(surface))),
            size: gfx::FloatSize::from(surface.get_size()),
        }
    }

    /// A texture that reports a size but owns nothing, for tests that want a size and no
    /// pixels. Layout only ever asks for `get_texture_size`, so this is enough to drive a
    /// `Button` or a `Sprite` headlessly.
    #[cfg(test)]
    #[must_use]
    pub const fn new_sized(size: gfx::FloatSize) -> Self {
        Self { handle: TextureHandle::NONE, size }
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
        let src_rect = src_rect.unwrap_or_else(|| Rect::new(gfx::FloatPos(0.0, 0.0), self.size));

        // Catches `Texture::new`, which is zero sized. A texture created without a device does
        // have a size, and the command it records is skipped by the backend when the id
        // resolves to nothing - which can only happen where there is no way to draw anyway.
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
            color: color.unwrap_or(Color::new(255, 255, 255, 255)),
        });
    }
}

/// Parks the GPU resource for release rather than dropping it outright, because a
/// `DrawCommand` recorded earlier this frame may still name this texture. Textures are
/// created and dropped inside `render_inner` all over the game, so that is the common case.
impl Drop for Texture {
    fn drop(&mut self) {
        if self.handle != TextureHandle::NONE {
            gpu_device::delete_texture_later(self.handle.0);
        }
    }
}
