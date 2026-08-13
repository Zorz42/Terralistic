//! What to draw, as plain data, with no idea of how it gets drawn.
//!
//! A primitive's `render` method *records* a `DrawCommand` into a `DrawList`, and something
//! that knows about a GPU - today `wgpu_backend::WgpuBackend` - replays the list once per
//! frame. So swapping the backend means writing one `execute` rather than rewriting every
//! widget, and a frame is inspectable: `DrawRecorder` is a `DrawTarget` with no window behind
//! it, which makes "what does this widget draw?" an ordinary `#[test]`.
//!
//! Coordinates in a command are window pixels, y-down from the top left, exactly as the
//! caller gave them. Clip space is the backend's business.
//!
//! Commands name textures and meshes by handle rather than by reference, because the list has
//! to outlive the borrow of whatever recorded into it. `Texture` and `VertexBuffer` still own
//! their GPU objects, so a handle can outlive its owner - see `gpu_device` for why that is
//! safe.

use crate::libraries::graphics as gfx;

/// How a draw combines with what is already in the framebuffer.
///
/// This is baked into a render pipeline rather than being a state switch, so the backend
/// keeps one pipeline per mode. Changing it mid-frame has to keep its place in the draw
/// order, which is why callers record a `SetBlendMode` command through
/// `DrawTarget::set_blend_mode` instead of calling anything directly.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BlendMode {
    Alpha,
    Multiply,
}

/// A texture living in the renderer backend.
///
/// Opaque outside the backend: today it is an index into `gpu_device`'s registry. The point
/// is that a command can name a resource without borrowing it.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct TextureHandle(pub(super) u32);

impl TextureHandle {
    /// The handle of a `Texture` that owns nothing on the GPU. Zero is never a real id.
    pub(super) const NONE: Self = Self(0);

    /// The backend's raw name for this resource. Only useful to a backend or a test.
    #[must_use]
    pub const fn get_id(self) -> u32 {
        self.0
    }
}

/// An uploaded triangle mesh living in the renderer backend.
///
/// Same contract as `TextureHandle`, except that the vertex count rides along so `execute`
/// can issue the draw without reaching back into the `VertexBuffer` that owns the resource.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct MeshHandle {
    pub(super) id: u32,
    pub(super) vertex_count: u32,
}

impl MeshHandle {
    /// The backend's raw id for this resource. Only useful to a backend or a test.
    #[must_use]
    pub const fn get_id(self) -> u32 {
        self.id
    }
}

/// One drawing operation, in window pixel coordinates.
///
/// The arguments are stored exactly as the caller passed them rather than pre-multiplied into
/// a destination rectangle, so the backend can rebuild the transform from the same inputs in
/// the same order and land on bit-identical floats. That is what keeps the golden images
/// exact.
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
/// `push_draw_command` takes `&self` because the toolkit is full of places that draw through
/// a shared borrow - `graphics.font.render_text(graphics, ..)` and
/// `graphics.shadow_context.render(graphics, ..)` both borrow the context twice. The
/// implementation keeps that working with a `RefCell`; the borrow is only ever held for the
/// length of a `push`, and nothing called from inside one can record.
pub trait DrawTarget {
    /// Records one command.
    fn push_draw_command(&self, command: DrawCommand);

    /// Size of the drawable area in logical pixels, for culling.
    fn get_draw_area(&self) -> gfx::FloatSize;

    /// Changes how everything recorded after this point blends with what is underneath.
    /// Recorded rather than applied, because it only means anything relative to the
    /// surrounding draw order.
    fn set_blend_mode(&self, blend_mode: BlendMode) {
        self.push_draw_command(DrawCommand::SetBlendMode(blend_mode));
    }
}

/// A `DrawTarget` with no GPU behind it, for tests. The draw-list counterpart of
/// `gfx::HeadlessContext`.
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
