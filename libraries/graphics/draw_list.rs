//! What to draw, as plain data, with no idea of how it gets drawn.
//!
//! A primitive's `render` *records* a `DrawCommand`, and the backend replays the list once per
//! frame. So swapping backends is one `execute` rather than every widget, and a frame is
//! inspectable: `DrawRecorder` is a `DrawTarget` with no window, which makes "what does this
//! widget draw?" an ordinary `#[test]`.
//!
//! Coordinates are window pixels, y-down from the top left, exactly as the caller gave them;
//! clip space is the backend's business. Textures and meshes are named by handle, because the
//! list outlives the borrow of whatever recorded into it - see `gpu_device`.

use crate::libraries::graphics as gfx;

/// How a draw combines with what is already in the framebuffer.
///
/// Baked into a pipeline rather than a state switch, and it has to keep its place in the draw order
/// - which is why callers record a `SetBlendMode` through `DrawTarget::set_blend_mode`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BlendMode {
    Alpha,
    Multiply,
}

/// A texture living in the renderer backend - an index into `gpu_device`'s registry, opaque
/// outside it. The point is that a command can name a resource without borrowing it.
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

/// An uploaded triangle mesh, on the same contract as `TextureHandle`. The vertex count stays
/// in the registry entry, which is what survives a `VertexBuffer` being replaced mid-frame.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct MeshHandle(pub(super) u32);

impl MeshHandle {
    /// The backend's raw id for this resource. Only useful to a backend or a test.
    #[must_use]
    pub const fn get_id(self) -> u32 {
        self.0
    }
}

/// One drawing operation, in window pixels.
///
/// The arguments are stored exactly as passed rather than pre-multiplied into a destination, so the
/// backend rebuilds the transform from the same inputs in the same order and lands on bit-identical
/// floats - which keeps the goldens exact.
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
/// `push_draw_command` takes `&self` because the toolkit draws through shared borrows -
/// `graphics.font.render_text(graphics, ..)` borrows the context twice - so implementations use a
/// `RefCell`, held only for the push itself.
pub trait DrawTarget {
    /// Records one command.
    fn push_draw_command(&self, command: DrawCommand);

    /// Size of the drawable area in logical pixels, for culling.
    fn get_draw_area(&self) -> gfx::FloatSize;

    /// Changes how everything after this blends. Recorded rather than applied, because it only
    /// means anything relative to the surrounding draw order.
    fn set_blend_mode(&self, blend_mode: BlendMode) {
        self.push_draw_command(DrawCommand::SetBlendMode(blend_mode));
    }
}

/// A `DrawTarget` with no GPU behind it, for tests. The draw-list counterpart of
/// `ui::HeadlessContext`.
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
