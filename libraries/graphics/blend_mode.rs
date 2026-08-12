#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BlendMode {
    Alpha,
    Multiply,
}

/// Applies a blend mode to the OpenGL state.
///
/// This is a backend operation, not something game code calls. Changing the blend mode
/// halfway through a frame has to keep its place in the draw order, so callers record a
/// `DrawCommand::SetBlendMode` through `DrawTarget::set_blend_mode` instead and the backend
/// ends up here when it replays that command.
pub(super) fn apply(blend_mode: BlendMode) {
    unsafe {
        match blend_mode {
            BlendMode::Alpha => {
                gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
            }
            BlendMode::Multiply => {
                gl::BlendFunc(gl::DST_COLOR, gl::ONE_MINUS_SRC_ALPHA);
            }
        }
    }
}
