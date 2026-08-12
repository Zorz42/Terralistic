//! What to draw, as plain data, with no idea of how it gets drawn.
//!
//! Every drawing primitive in this toolkit used to issue OpenGL calls directly from its
//! `render` method, which meant the game's whole render path was welded to one backend and
//! could only be checked by taking a screenshot. Now `render` *records* a `DrawCommand`
//! into a `DrawList`, and something that knows about a GPU - today `gl_backend::GlBackend`
//! - replays the list once per frame.
//!
//! Two things fall out of that:
//!
//! - Swapping the backend means writing one `execute`, not rewriting every widget.
//! - A frame is inspectable. `DrawRecorder` is a `DrawTarget` with no window behind it, so
//!   "what does this widget draw?" is an ordinary `#[test]` rather than a golden image.
//!
//! Coordinates in a command are window pixels, y-down from the top left, exactly as the
//! caller gave them. Nothing here knows about clip space; that is the backend's business.
//!
//! # Resources
//!
//! Commands refer to textures and meshes by handle rather than by reference, because the
//! list has to outlive the borrow of whatever recorded into it. The handle carries the
//! backend's own name for the resource, which is the single place a backend-specific value
//! crosses this boundary. `Texture` and `VertexBuffer` still own their GPU objects, so a
//! handle can outlive its owner - see `gpu_garbage` for why that is safe.

use crate::libraries::graphics as gfx;

use super::blend_mode::BlendMode;

/// A texture living in the renderer backend.
///
/// Opaque outside `gl_backend`: today it holds an OpenGL texture name, a wgpu backend would
/// put its own index here. `Texture::new` produces `NONE`, which never reaches a draw
/// because a texture with no GPU object also reports a zero size and gets culled first.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct TextureHandle(pub(super) u32);

impl TextureHandle {
    /// The handle of a `Texture` that owns nothing on the GPU.
    pub(super) const NONE: Self = Self(u32::MAX);

    /// The backend's raw name for this resource. Only useful to a backend or a test.
    #[must_use]
    pub const fn get_id(self) -> u32 {
        self.0
    }
}

/// An uploaded triangle mesh living in the renderer backend.
///
/// Same contract as `TextureHandle`: the fields are the backend's business. They are three
/// OpenGL object names plus the element count, which is everything `execute` needs to issue
/// the draw without reaching back into the `VertexBuffer` that owns them.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct MeshHandle {
    pub(super) vertex_array: u32,
    pub(super) vertex_buffer: u32,
    pub(super) index_buffer: u32,
    pub(super) index_count: u32,
}

impl MeshHandle {
    /// How many indices the mesh will draw. Zero means the mesh is empty.
    #[must_use]
    pub const fn get_index_count(self) -> u32 {
        self.index_count
    }
}

/// One drawing operation, in window pixel coordinates.
///
/// The arguments are stored exactly as the caller passed them rather than pre-multiplied
/// into a destination rectangle. That is deliberate: the backend rebuilds the transform
/// from the same inputs in the same order, so the floating point result is bit-identical to
/// what the immediate mode code produced, which is what keeps the golden images exact.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum DrawCommand {
    /// A filled rectangle.
    Rect { rect: gfx::Rect, color: gfx::Color },
    /// The four edges of a rectangle, one pixel wide.
    RectOutline { rect: gfx::Rect, color: gfx::Color },
    /// A region of a texture, scaled and tinted.
    Texture {
        texture: TextureHandle,
        /// Size of the whole texture, which is what source coordinates are relative to.
        texture_size: gfx::FloatSize,
        src_rect: gfx::Rect,
        pos: gfx::FloatPos,
        scale: f32,
        flipped: bool,
        color: gfx::Color,
    },
    /// A mesh with its own per-vertex colours and texture coordinates, offset by `pos`.
    Mesh {
        mesh: MeshHandle,
        /// The texture to sample and its size, or `None` for flat vertex colours.
        texture: Option<(TextureHandle, gfx::FloatSize)>,
        pos: gfx::FloatPos,
    },
    /// Blurs what has already been drawn inside `rect`. Order matters: everything recorded
    /// before this is blurred, everything after is not.
    Blur { rect: gfx::Rect, radius: i32 },
    /// Changes how subsequent commands combine with the framebuffer.
    SetBlendMode(BlendMode),
}

/// One frame's worth of drawing, in the order it was recorded.
#[derive(Default)]
pub struct DrawList {
    commands: Vec<DrawCommand>,
}

impl DrawList {
    #[must_use]
    pub const fn new() -> Self {
        Self { commands: Vec::new() }
    }

    pub fn push(&mut self, command: DrawCommand) {
        self.commands.push(command);
    }

    #[must_use]
    pub fn get_commands(&self) -> &[DrawCommand] {
        &self.commands
    }

    /// Empties the list but keeps its allocation, so a steady state frame does not allocate.
    pub fn clear(&mut self) {
        self.commands.clear();
    }

    #[must_use]
    pub const fn len(&self) -> usize {
        self.commands.len()
    }

    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.commands.is_empty()
    }
}

/// Somewhere drawing commands can be recorded.
///
/// `push_draw_command` takes `&self` rather than `&mut self` because the toolkit is full of
/// places that draw through a shared borrow - `graphics.font.render_text(graphics, ..)` and
/// `graphics.shadow_context.render(graphics, ..)` both borrow the context twice. That was
/// fine when drawing went straight to OpenGL, which is a mutable global by nature, and the
/// implementation keeps it fine with a `RefCell`. The borrow is only ever held for the
/// length of a `push`, and nothing called from inside one can record.
pub trait DrawTarget {
    /// Records one command.
    fn push_draw_command(&self, command: DrawCommand);

    /// Size of the drawable area in logical pixels, for culling.
    fn get_draw_area(&self) -> gfx::FloatSize;

    /// Changes how everything recorded after this point blends with what is underneath.
    ///
    /// This has to be recorded rather than applied, because it only means anything relative
    /// to the surrounding draw order.
    fn set_blend_mode(&self, blend_mode: BlendMode) {
        self.push_draw_command(DrawCommand::SetBlendMode(blend_mode));
    }
}

/// A `DrawTarget` with no GPU behind it, for tests.
///
/// This is the draw-list counterpart of `gfx::HeadlessContext`: it makes what a primitive or
/// a widget draws assertable in `cargo test`, where the golden-image suite cannot run
/// because it needs a real OpenGL context on the main thread.
#[cfg(test)]
pub struct DrawRecorder {
    commands: std::cell::RefCell<DrawList>,
    draw_area: gfx::FloatSize,
}

#[cfg(test)]
impl DrawRecorder {
    /// A recorder with a 320x240 drawable area, matching the golden-image window.
    #[must_use]
    pub fn new() -> Self {
        Self::with_draw_area(gfx::FloatSize(320.0, 240.0))
    }

    #[must_use]
    pub fn with_draw_area(draw_area: gfx::FloatSize) -> Self {
        Self {
            commands: std::cell::RefCell::new(DrawList::new()),
            draw_area,
        }
    }

    /// Everything recorded so far, in order.
    #[must_use]
    pub fn get_commands(&self) -> Vec<DrawCommand> {
        self.commands.borrow().get_commands().to_vec()
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.commands.borrow().len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.commands.borrow().is_empty()
    }
}

#[cfg(test)]
impl DrawTarget for DrawRecorder {
    fn push_draw_command(&self, command: DrawCommand) {
        self.commands.borrow_mut().push(command);
    }

    fn get_draw_area(&self) -> gfx::FloatSize {
        self.draw_area
    }
}
