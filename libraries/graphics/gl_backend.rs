//! The OpenGL half of the renderer: how a `DrawList` becomes pixels.
//!
//! Everything in this file is replaceable. It owns the shaders, the offscreen framebuffer
//! and the blur pass, and its only job is to take the backend-agnostic commands that
//! `draw_list` describes and issue the OpenGL calls that carry them out. A different
//! backend would reimplement `execute`, `resize` and `present` and leave the rest of the
//! toolkit alone.
//!
//! # Why the transforms are rebuilt here rather than stored in the command
//!
//! A command carries the arguments the caller passed - a position, a scale, a source
//! rectangle - not a finished destination rectangle. Clip space is an OpenGL notion: the
//! y flip, the divide by the window size and the order the multiplications happen in all
//! belong to this file. Keeping the arithmetic in one place is also what makes the change
//! from immediate mode to a command list bit-exact, since the same floats are combined in
//! the same order as before.

use anyhow::Result;

use crate::libraries::graphics as gfx;

use super::blend_mode::{self, BlendMode};
use super::blur::BlurContext;
use super::draw_list::{DrawCommand, DrawList, MeshHandle};
use super::gpu_garbage;
use super::passthrough_shader::PassthroughShader;
use super::transformation::Transformation;
use super::vertex_buffer::{draw_mesh, DrawMode};

/// Owns every OpenGL object the renderer needs for a frame.
pub struct GlBackend {
    passthrough_shader: PassthroughShader,
    blur_context: BlurContext,
    /// What the game draws into. Only `present` copies it to the actual window.
    window_texture: u32,
    /// Scratch for the blur's two passes, which ping-pong between the two textures.
    window_texture_back: u32,
    window_framebuffer: u32,
    /// Window pixels to clip space. Recomputed whenever the drawable size changes.
    normalization_transform: Transformation,
}

impl GlBackend {
    /// Sets up the shaders and the offscreen framebuffer. An OpenGL context must already be
    /// current.
    pub(super) fn new() -> Result<Self> {
        unsafe {
            gl::Enable(gl::BLEND);
        }
        blend_mode::apply(BlendMode::Alpha);

        let passthrough_shader = PassthroughShader::new()?;
        let mut window_texture = 0;
        let mut window_texture_back = 0;
        let mut window_framebuffer = 0;

        unsafe {
            gl::GenTextures(1, &raw mut window_texture);
            gl::GenTextures(1, &raw mut window_texture_back);
            gl::GenFramebuffers(1, &raw mut window_framebuffer);
            gl::BindFramebuffer(gl::FRAMEBUFFER, window_framebuffer);
        }

        Ok(Self {
            passthrough_shader,
            blur_context: BlurContext::new()?,
            window_texture,
            window_texture_back,
            window_framebuffer,
            normalization_transform: Transformation::new(),
        })
    }

    /// Reallocates the offscreen textures for a new drawable size.
    pub(super) fn resize(&self, size: gfx::IntSize) {
        for texture in [self.window_texture, self.window_texture_back] {
            unsafe {
                gl::BindTexture(gl::TEXTURE_2D, texture);
                gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGBA as i32, size.0 as i32, size.1 as i32, 0, gl::BGRA, gl::UNSIGNED_BYTE, std::ptr::null());

                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
            }
        }

        unsafe {
            gl::Viewport(0, 0, size.0 as i32, size.1 as i32);
        }
    }

    /// Recomputes the transform that maps window pixel coordinates onto OpenGL clip space.
    ///
    /// The negative y scale is OpenGL's convention: clip space is y-up with the framebuffer
    /// origin at the bottom left, while every coordinate in this toolkit is y-down from the
    /// top left.
    pub(super) fn update_normalization_transform(&mut self, window_size: gfx::FloatSize) {
        self.normalization_transform = Transformation::new();
        self.normalization_transform.translate(gfx::FloatPos(-1.0, 1.0));
        self.normalization_transform.stretch((2.0 / window_size.0, -2.0 / window_size.1));
    }

    /// Draws a whole frame, then reclaims anything the frame dropped.
    ///
    /// The list is self-contained: the shader program and the blend mode are set here rather
    /// than assumed, so a frame never inherits state from the one before it. That used to be
    /// left to chance - nothing bound the passthrough program at startup, and the game only
    /// rendered at all because the first `RenderRect` of the first frame happened to bind it
    /// on its way out of a blur that did nothing.
    pub(super) fn execute(&mut self, list: &DrawList, window_size: gfx::FloatSize) {
        unsafe {
            gl::UseProgram(self.passthrough_shader.passthrough_shader);
        }
        blend_mode::apply(BlendMode::Alpha);

        for command in list.get_commands() {
            self.execute_command(command, window_size);
        }

        gpu_garbage::collect();
    }

    fn execute_command(&self, command: &DrawCommand, window_size: gfx::FloatSize) {
        match *command {
            DrawCommand::Rect { rect, color } => self.draw_rect(rect, color, false),
            DrawCommand::RectOutline { rect, color } => self.draw_rect(rect, color, true),
            DrawCommand::Texture {
                texture,
                texture_size,
                src_rect,
                pos,
                scale,
                flipped,
                color,
            } => self.draw_texture(texture.get_id(), texture_size, src_rect, pos, scale, flipped, color),
            DrawCommand::Mesh { mesh, texture, pos } => self.draw_mesh_command(mesh, texture, pos),
            DrawCommand::Blur { rect, radius } => self.draw_blur(rect, radius, window_size),
            DrawCommand::SetBlendMode(blend_mode) => blend_mode::apply(blend_mode),
        }
    }

    fn set_color_uniform(&self, color: gfx::Color) {
        unsafe {
            gl::Uniform4f(
                self.passthrough_shader.global_color,
                color.r as f32 / 255.0,
                color.g as f32 / 255.0,
                color.b as f32 / 255.0,
                color.a as f32 / 255.0,
            );
        }
    }

    fn draw_rect(&self, rect: gfx::Rect, color: gfx::Color, outline: bool) {
        let mut transform = self.normalization_transform.clone();
        transform.translate(rect.pos);
        transform.stretch((rect.size.0, rect.size.1));

        unsafe {
            gl::UniformMatrix3fv(self.passthrough_shader.transform_matrix, 1, gl::FALSE, transform.matrix.as_ptr());
        }
        self.set_color_uniform(color);
        unsafe {
            gl::Uniform1i(self.passthrough_shader.has_texture, 0);
        }

        if outline {
            self.passthrough_shader.rect_outline_vertex_buffer.draw(false, DrawMode::Lines);
        } else {
            self.passthrough_shader.rect_vertex_buffer.draw(false, DrawMode::Triangles);
        }
    }

    fn draw_texture(&self, texture: u32, texture_size: gfx::FloatSize, src_rect: gfx::Rect, pos: gfx::FloatPos, scale: f32, flipped: bool, color: gfx::Color) {
        let mut transform = self.normalization_transform.clone();

        if flipped {
            transform.translate(gfx::FloatPos(src_rect.size.0 * scale + pos.0 * 2.0, 0.0));
            transform.stretch((-1.0, 1.0));
        }

        transform.translate(pos);
        transform.stretch((src_rect.size.0 * scale, src_rect.size.1 * scale));

        unsafe {
            gl::UniformMatrix3fv(self.passthrough_shader.transform_matrix, 1, gl::FALSE, transform.matrix.as_ptr());

            // Texture coordinates are the [0,1] unit square, mapped onto the source
            // rectangle and then divided by the texture size. The stretch is by exactly
            // `src_rect.size`: sampling happens at pixel centres, so the last output column
            // already lands strictly inside the region. There used to be a `+ 0.1` fudge
            // here, and it pushed that column into the neighbouring texel.
            transform = normalization_transform_for(texture_size);
            transform.translate(src_rect.pos);
            transform.stretch((src_rect.size.0, src_rect.size.1));

            gl::UniformMatrix3fv(self.passthrough_shader.texture_transform_matrix, 1, gl::FALSE, transform.matrix.as_ptr());
        }
        self.set_color_uniform(color);
        unsafe {
            gl::Uniform1i(self.passthrough_shader.has_texture, 1);
            gl::BindTexture(gl::TEXTURE_2D, texture);
        }

        self.passthrough_shader.rect_vertex_buffer.draw(true, DrawMode::Triangles);
    }

    fn draw_mesh_command(&self, mesh: MeshHandle, texture: Option<(gfx::TextureHandle, gfx::FloatSize)>, pos: gfx::FloatPos) {
        // to avoid artifacts
        let pos = gfx::FloatPos(pos.0 + 0.01, pos.1 + 0.01);

        let mut transform = self.normalization_transform.clone();
        transform.translate(pos);

        unsafe {
            gl::UniformMatrix3fv(self.passthrough_shader.transform_matrix, 1, gl::FALSE, transform.matrix.as_ptr());

            match texture {
                None => gl::Uniform1i(self.passthrough_shader.has_texture, 0),
                Some((handle, size)) => {
                    let texture_transform = normalization_transform_for(size);
                    gl::UniformMatrix3fv(self.passthrough_shader.texture_transform_matrix, 1, gl::FALSE, texture_transform.matrix.as_ptr());
                    gl::Uniform1i(self.passthrough_shader.has_texture, 1);
                    gl::BindTexture(gl::TEXTURE_2D, handle.get_id());
                }
            }

            gl::Uniform4f(self.passthrough_shader.global_color, 1.0, 1.0, 1.0, 1.0);
        }

        draw_mesh(mesh, texture.is_some(), DrawMode::Triangles);
    }

    fn draw_blur(&self, rect: gfx::Rect, radius: i32, window_size: gfx::FloatSize) {
        self.blur_context
            .blur_region(rect, radius, self.window_texture, self.window_texture_back, window_size, &self.normalization_transform);
        unsafe {
            gl::UseProgram(self.passthrough_shader.passthrough_shader);
        }
    }

    /// Advances the blur fade by however many 10 ms frames have elapsed.
    pub(super) fn update_blur(&mut self) {
        self.blur_context.update();
    }

    pub(super) const fn set_blur_enabled(&mut self, enable: bool) {
        self.blur_context.blur_enabled = enable;
    }

    /// Copies the offscreen texture to the window's own framebuffer.
    pub(super) fn present(&self, window_size: gfx::FloatSize) {
        unsafe {
            gl::BindFramebuffer(gl::READ_FRAMEBUFFER, self.window_framebuffer);
            gl::FramebufferTexture2D(gl::READ_FRAMEBUFFER, gl::COLOR_ATTACHMENT0, gl::TEXTURE_2D, self.window_texture, 0);
            gl::BindFramebuffer(gl::DRAW_FRAMEBUFFER, 0);

            // This used to be three byte-identical copies of the same call, one each
            // under cfg(windows), cfg(macos) and cfg(linux) - which also meant no blit at
            // all on any other target.
            //
            // The 2.0 is a hardcoded assumption that the drawable is twice the window
            // size. That is wrong on non-HiDPI displays, but fixing it needs the real
            // drawable size and a visual check, so it is left as is here.
            let blit_width = (window_size.0 * 2.0) as i32;
            let blit_height = (window_size.1 * 2.0) as i32;
            gl::BlitFramebuffer(0, 0, blit_width, blit_height, 0, 0, blit_width, blit_height, gl::COLOR_BUFFER_BIT, gl::NEAREST);
        }
    }

    /// Points OpenGL back at the offscreen framebuffer, after `present` sent it elsewhere.
    pub(super) fn bind_offscreen_framebuffer(&self) {
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, self.window_framebuffer);
        }
    }

    /// Attaches the offscreen texture and clears it, so a captured frame starts from a
    /// known buffer.
    ///
    /// `new` generates the framebuffer but never attaches a texture to it - that only
    /// happens on the first `present`. The golden-image tests never present (there is
    /// nothing to show on a hidden window), so they attach it here.
    #[cfg(feature = "render-tests")]
    pub(super) fn begin_capture_frame(&mut self, size: gfx::IntSize) {
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, self.window_framebuffer);
            gl::FramebufferTexture2D(gl::FRAMEBUFFER, gl::COLOR_ATTACHMENT0, gl::TEXTURE_2D, self.window_texture, 0);
            gl::Viewport(0, 0, size.0 as i32, size.1 as i32);
            gl::ClearColor(0.0, 0.0, 0.0, 0.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::UseProgram(self.passthrough_shader.passthrough_shader);
        }
    }

    /// Reads the offscreen framebuffer back into a `Surface`.
    ///
    /// This is upstream of the `HiDPI` blit in `present`, whose hardcoded 2.0 would
    /// otherwise contaminate every golden.
    #[cfg(feature = "render-tests")]
    pub(super) fn read_pixels(&self, size: gfx::IntSize) -> gfx::Surface {
        let mut flipped = gfx::Surface::new(size);

        unsafe {
            gl::Finish();
            gl::BindFramebuffer(gl::READ_FRAMEBUFFER, self.window_framebuffer);
            gl::FramebufferTexture2D(gl::READ_FRAMEBUFFER, gl::COLOR_ATTACHMENT0, gl::TEXTURE_2D, self.window_texture, 0);
            // Color is four u8s in r, g, b, a order, which is the same assumption
            // Texture::load_from_surface already makes when it uploads a surface.
            gl::ReadPixels(0, 0, size.0 as i32, size.1 as i32, gl::RGBA, gl::UNSIGNED_BYTE, flipped.pixels.as_mut_ptr().cast::<std::ffi::c_void>());
        }

        // OpenGL hands back rows bottom to top; Surface is top to bottom.
        let mut result = gfx::Surface::new(size);
        for (pos, pixel) in result.iter_mut() {
            if let Ok(source) = flipped.get_pixel(gfx::IntPos(pos.0, size.1 as i32 - 1 - pos.1)) {
                *pixel = *source;
            }
        }
        result
    }

    /// Jumps the blur fade to its target, so a golden does not depend on how many 10 ms
    /// frames happened to elapse before the capture.
    #[cfg(feature = "render-tests")]
    pub(super) const fn settle_blur(&mut self) {
        self.blur_context.settle();
    }
}

/// Maps texel coordinates onto the `[0,1]` range the sampler wants.
fn normalization_transform_for(texture_size: gfx::FloatSize) -> Transformation {
    let mut result = Transformation::new();
    result.stretch((1.0 / texture_size.0, 1.0 / texture_size.1));
    result
}

impl Drop for GlBackend {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteFramebuffers(1, &raw const self.window_framebuffer);
            gl::DeleteTextures(1, &raw const self.window_texture);
            gl::DeleteTextures(1, &raw const self.window_texture_back);
        }
    }
}
