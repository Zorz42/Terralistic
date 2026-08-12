/// How a draw combines with what is already in the framebuffer.
///
/// Under wgpu this is baked into a render pipeline rather than being a state switch, so the
/// backend keeps one pipeline per mode. Changing it mid-frame still has to keep its place in
/// the draw order, which is why callers record a `DrawCommand::SetBlendMode` through
/// `DrawTarget::set_blend_mode` instead of calling anything directly.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BlendMode {
    Alpha,
    Multiply,
}
