pub enum BlendMode {
    Alpha,
    Multiply,
}

pub fn set_blend_mode(blend_mode: BlendMode) {
    // Safety: OpenGL functions are unsafe
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
