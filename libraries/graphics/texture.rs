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

    /// Uploads a surface to the GPU. With no device - `cargo test`, or any process that never
    /// opened a window - the result knows its size and owns nothing, so layout still works and
    /// drawing it is a no-op.
    #[must_use]
    pub fn load_from_surface(surface: &Surface) -> Self {
        Self {
            handle: gpu_device::get().map_or(TextureHandle::NONE, |gpu| TextureHandle(gpu.create_texture(surface))),
            size: gfx::FloatSize::from(surface.get_size()),
        }
    }

    /// Uploads a serialized `Surface` - an `.opa` file's bytes, `include_bytes!`-ed or pulled
    /// from a module's resources. **One that will not decode becomes an empty texture, not an
    /// error**: a corrupt asset should leave a hole in the screen rather than take the process
    /// down, and the ten call sites that spelled this out invented three fallback sizes.
    #[must_use]
    pub fn load_from_bytes(bytes: &[u8]) -> Self {
        Self::load_from_surface(&Surface::deserialize_from_bytes(bytes).unwrap_or_else(|_| Surface::new(gfx::IntSize(1, 1))))
    }

    /// A texture that reports a size and owns nothing. Layout only asks for
    /// `get_texture_size`, so this drives a `Button` or a `Sprite` headlessly.
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

        // Catches `Texture::new`, which is zero sized. One created without a device does have
        // a size; the backend skips its command when the id resolves to nothing.
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

/// Parks the GPU resource rather than releasing it: a `DrawCommand` recorded earlier this
/// frame may still name it, which is the common case - textures are created and dropped inside
/// `render_inner` all over the game.
impl Drop for Texture {
    fn drop(&mut self) {
        if self.handle != TextureHandle::NONE {
            gpu_device::delete_texture_later(self.handle.0);
        }
    }
}
