use crate::libraries::graphics as gfx;

use super::draw_list::{DrawCommand, DrawTarget, TextureHandle};
use super::gpu_device;
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

    /// Uploads a Surface to the GPU.
    ///
    /// With no device - a process that never opened a window, such as `cargo test` - this
    /// returns a texture that knows its size but owns nothing, so layout code still works
    /// and drawing it is a no-op. Under OpenGL the same call without a context was undefined
    /// behaviour.
    #[must_use]
    pub fn load_from_surface(surface: &Surface) -> Self {
        Self {
            handle: gpu_device::get().map_or(TextureHandle::NONE, |gpu| TextureHandle(gpu.create_texture(surface))),
            size: gfx::FloatSize::from(surface.get_size()),
        }
    }

    /// A texture that reports a size but owns nothing on the GPU, for tests.
    ///
    /// Layout code only ever asks a texture for `get_texture_size`, so this is enough to
    /// drive a `Button` or a `Sprite` headlessly. It is safe to drop: the handle stays
    /// `TextureHandle::NONE`, so `free_texture` has nothing to park. `load_from_surface`
    /// also works without a device now, so this is only for tests that want a size and no
    /// pixels.
    #[cfg(test)]
    #[must_use]
    pub const fn new_sized(size: gfx::FloatSize) -> Self {
        Self { handle: TextureHandle::NONE, size }
    }

    /// Parks the GPU resource for release if there is one.
    ///
    /// The id goes to `gpu_device`'s pending list rather than being removed outright,
    /// because a `DrawCommand` recorded earlier this frame may still name this texture.
    /// Textures are created and dropped inside `render_inner` all over the game, so this is
    /// the common case, not a corner one.
    fn free_texture(&mut self) {
        if self.handle != TextureHandle::NONE {
            gpu_device::delete_texture_later(self.handle.0);
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

        // A `Texture::new` has a zero size, so this catches it too. A texture created
        // without a device does have a size, and the command it records is skipped by the
        // backend when the id resolves to nothing - which can only happen in a process that
        // has no way to draw in the first place.
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
