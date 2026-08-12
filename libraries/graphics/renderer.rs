use std::collections::HashMap;
use std::collections::VecDeque;
use std::mem::swap;

use arboard::Clipboard;

use anyhow::{anyhow, Result};
use sdl2::video::SwapInterval;

use crate::libraries::graphics as gfx;
use crate::libraries::graphics::blur::BlurContext;
use crate::libraries::graphics::events::sdl_event_to_gfx_event;
use crate::libraries::graphics::passthrough_shader::PassthroughShader;
use crate::libraries::graphics::shadow::ShadowContext;
use crate::libraries::graphics::transformation::Transformation;
use crate::libraries::graphics::Font;
use crate::libraries::graphics::UiContext;

/// This stores all the values needed for rendering.
pub struct GraphicsContext {
    _gl_context: sdl2::video::GLContext,
    sdl_window: sdl2::video::Window,
    sdl_event_pump: sdl2::EventPump,
    video_subsystem: sdl2::VideoSubsystem,
    pub(super) normalization_transform: Transformation,
    window_texture: u32,
    window_texture_back: u32,
    window_framebuffer: u32,
    blur_context: BlurContext,
    pub(super) passthrough_shader: PassthroughShader,
    events_queue: VecDeque<gfx::Event>,
    window_open: bool,
    // Keep track of all Key states as a hashmap
    key_states: HashMap<gfx::Key, bool>,
    events: Vec<gfx::Event>,
    pub(super) shadow_context: ShadowContext,
    clipboard_context: Clipboard,
    pub block_key_states: bool,
    pub scale: f32,
    real_scale: f32,
    scale_animation_timer: gfx::AnimationTimer,
    min_ms_per_frame: f32,
    frames_so_far: u32,
    ms_so_far: f32,
    prev_frame_time: std::time::Instant,
    pub font: Font,
    pub font_mono: Option<Font>,
}

impl GraphicsContext {
    /// Initializes all the values needed for rendering.
    /// It usually fails because the system doesn't support graphics.
    pub fn new(window_width: u32, window_height: u32, window_title: &str, font: &[u8], font_mono: Option<&[u8]>) -> Result<Self> {
        Self::new_with_visibility(window_width, window_height, window_title, font, font_mono, true)
    }

    /// Same as `new`, but the window is never mapped on screen.
    ///
    /// Rendering already goes to `window_texture` rather than the default framebuffer, so
    /// a hidden window is enough to drive the whole renderer - only the final blit in
    /// `update_window` needs a visible one. This is what the golden-image tests use, so
    /// running them does not flash windows across the desktop.
    #[cfg(feature = "render-tests")]
    pub fn new_hidden(window_width: u32, window_height: u32, font: &[u8], font_mono: Option<&[u8]>) -> Result<Self> {
        Self::new_with_visibility(window_width, window_height, "Terralistic render tests", font, font_mono, false)
    }

    fn new_with_visibility(window_width: u32, window_height: u32, window_title: &str, font: &[u8], font_mono: Option<&[u8]>, visible: bool) -> Result<Self> {
        let sdl = sdl2::init();
        let sdl = sdl.map_err(|e| anyhow!(e))?;
        let video_subsystem = sdl.video();
        let video_subsystem = video_subsystem.map_err(|e| anyhow!(e))?;

        let gl_attr = video_subsystem.gl_attr();

        gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
        gl_attr.set_context_version(3, 3);

        let mut window_builder = video_subsystem.window(window_title, window_width, window_height);
        window_builder.position_centered().opengl().resizable();
        if !visible {
            window_builder.hidden();
        }
        let sdl_window = window_builder.build()?;

        let gl_context = sdl_window.gl_create_context().map_err(|e| anyhow!(e))?;
        gl::load_with(|s| video_subsystem.gl_get_proc_address(s).cast::<std::ffi::c_void>());

        unsafe {
            gl::Enable(gl::BLEND);
        }
        gfx::set_blend_mode(gfx::BlendMode::Alpha);

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

        let shadow_context = ShadowContext::new();

        let font = Font::new(font, false)?;
        let font_mono = if let Some(data) = font_mono { Some(Font::new(data, true)?) } else { None };

        let mut result = Self {
            _gl_context: gl_context,
            sdl_window,
            sdl_event_pump: sdl.event_pump().map_err(|e| anyhow!(e))?,
            video_subsystem,
            normalization_transform: Transformation::new(),
            window_texture,
            window_texture_back,
            window_framebuffer,
            key_states: HashMap::new(),
            events: Vec::new(),
            blur_context: BlurContext::new()?,
            passthrough_shader,
            shadow_context,
            events_queue: VecDeque::new(),
            window_open: true,
            clipboard_context: Clipboard::new()?,
            block_key_states: false,
            scale: 1.0,
            real_scale: 1.0,
            scale_animation_timer: gfx::AnimationTimer::new(10),
            min_ms_per_frame: 0.0,
            frames_so_far: 0,
            ms_so_far: 0.0,
            prev_frame_time: std::time::Instant::now(),
            font,
            font_mono,
        };

        result.handle_window_resize();

        Ok(result)
    }

    /// Is called every time the window is resized.
    pub fn handle_window_resize(&mut self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.window_texture);
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA as i32,
                self.sdl_window.size().0 as i32,
                self.sdl_window.size().1 as i32,
                0,
                gl::BGRA as gl::types::GLenum,
                gl::UNSIGNED_BYTE,
                std::ptr::null(),
            );

            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);

            gl::BindTexture(gl::TEXTURE_2D, self.window_texture_back);
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA as i32,
                self.sdl_window.size().0 as i32,
                self.sdl_window.size().1 as i32,
                0,
                gl::BGRA,
                gl::UNSIGNED_BYTE,
                std::ptr::null(),
            );

            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);

            gl::Viewport(0, 0, self.sdl_window.size().0 as i32, self.sdl_window.size().1 as i32);
        }
    }

    /// Recomputes the transform that maps window pixel coordinates onto OpenGL clip space.
    ///
    /// The negative y scale is OpenGL's convention: clip space is y-up with the framebuffer
    /// origin at the bottom left, while every coordinate in this toolkit is y-down from the
    /// top left.
    fn update_normalization_transform(&mut self) {
        self.normalization_transform = Transformation::new();
        self.normalization_transform.translate(gfx::FloatPos(-1.0, 1.0));
        self.normalization_transform.stretch((2.0 / self.get_window_size().0, -2.0 / self.get_window_size().1));
    }

    /// Prepares a deterministic offscreen frame for the golden-image tests.
    ///
    /// `new` generates the framebuffer but never attaches a texture to it - that only
    /// happens on the first `update_window`. The tests never call `update_window` (there is
    /// nothing to present to on a hidden window), so they attach it here, and clear to
    /// transparent so every case starts from an identical buffer.
    #[cfg(feature = "render-tests")]
    pub fn begin_capture_frame(&mut self) {
        self.update_normalization_transform();
        let size = self.sdl_window.size();

        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, self.window_framebuffer);
            gl::FramebufferTexture2D(gl::FRAMEBUFFER, gl::COLOR_ATTACHMENT0, gl::TEXTURE_2D, self.window_texture, 0);
            gl::Viewport(0, 0, size.0 as i32, size.1 as i32);
            gl::ClearColor(0.0, 0.0, 0.0, 0.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::UseProgram(self.passthrough_shader.passthrough_shader);
        }
    }

    /// Reads the offscreen frame back into a `Surface`.
    ///
    /// The read is from `window_framebuffer`, so this captures what was drawn rather than
    /// what reached the screen - it is deliberately upstream of the `HiDPI` blit in
    /// `update_window`, whose hardcoded 2.0 would otherwise contaminate every golden.
    #[cfg(feature = "render-tests")]
    #[must_use]
    pub fn capture_frame(&self) -> gfx::Surface {
        let size = self.sdl_window.size();
        let mut flipped = gfx::Surface::new(gfx::IntSize(size.0, size.1));

        unsafe {
            gl::Finish();
            gl::BindFramebuffer(gl::READ_FRAMEBUFFER, self.window_framebuffer);
            gl::FramebufferTexture2D(gl::READ_FRAMEBUFFER, gl::COLOR_ATTACHMENT0, gl::TEXTURE_2D, self.window_texture, 0);
            // Color is four u8s in r, g, b, a order, which is the same assumption
            // Texture::load_from_surface already makes when it uploads a surface.
            gl::ReadPixels(0, 0, size.0 as i32, size.1 as i32, gl::RGBA, gl::UNSIGNED_BYTE, flipped.pixels.as_mut_ptr().cast::<std::ffi::c_void>());
        }

        // OpenGL hands back rows bottom to top; Surface is top to bottom.
        let mut result = gfx::Surface::new(gfx::IntSize(size.0, size.1));
        for (pos, pixel) in result.iter_mut() {
            if let Ok(source) = flipped.get_pixel(gfx::IntPos(pos.0, size.1 as i32 - 1 - pos.1)) {
                *pixel = *source;
            }
        }
        result
    }

    /// Jumps the scale and blur animations straight to their target values.
    ///
    /// Both are driven by wall-clock timers, so without this a golden would depend on how
    /// long the test happened to take.
    #[cfg(feature = "render-tests")]
    pub const fn settle_animations(&mut self) {
        self.real_scale = self.scale;
        self.blur_context.settle();
    }

    /// Returns an array of events, such as key presses.
    fn get_events(&mut self) -> Vec<gfx::Event> {
        let mut sdl_events = vec![];

        for sdl_event in self.sdl_event_pump.poll_iter() {
            sdl_events.push(sdl_event);
        }

        for sdl_event in sdl_events {
            match sdl_event {
                // handle window resize
                sdl2::event::Event::Window {
                    win_event: sdl2::event::WindowEvent::Resized(_width, _height),
                    ..
                } => {
                    self.handle_window_resize();
                }
                // handle quit event
                sdl2::event::Event::Quit { .. } => {
                    self.close_window();
                }
                _ => {}
            }

            if let Some(event) = sdl_event_to_gfx_event(&sdl_event) {
                // if event is a key press event update the key states to true
                if let gfx::Event::KeyPress(key, ..) = event {
                    self.set_key_state(key, true);
                }
                // if event is a key release event update the key states to false
                if let gfx::Event::KeyRelease(key, ..) = event {
                    self.set_key_state(key, false);
                }

                self.events.push(event);
            }
        }

        let mut result = Vec::new();
        swap(&mut result, &mut self.events);

        result
    }

    /// Returns an event, returns None if there are no events
    pub fn get_event(&mut self) -> Option<gfx::Event> {
        for event in self.get_events() {
            self.events_queue.push_back(event);
        }
        self.events_queue.pop_front()
    }

    /// Checks if the window is open, this becomes false, when the user closes the window, or the program closes it
    #[must_use]
    pub const fn is_window_open(&self) -> bool {
        self.window_open
    }

    /// Closes the window
    pub const fn close_window(&mut self) {
        self.window_open = false;
    }

    /// Should be called after rendering
    pub fn update_window(&mut self) {
        self.blur_context.update();

        while self.scale_animation_timer.frame_ready() {
            self.real_scale += (self.scale - self.real_scale) / 10.0;
            if f32::abs(self.real_scale - self.scale) < 0.001 {
                self.real_scale = self.scale;
            }
        }

        self.update_normalization_transform();

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
            let blit_width = (self.get_window_size().0 * 2.0) as i32;
            let blit_height = (self.get_window_size().1 * 2.0) as i32;
            gl::BlitFramebuffer(0, 0, blit_width, blit_height, 0, 0, blit_width, blit_height, gl::COLOR_BUFFER_BIT, gl::NEAREST);
        }

        self.sdl_window.gl_swap_window();

        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, self.window_framebuffer);
        }

        self.frames_so_far += 1;
        let now = std::time::Instant::now();
        let delta = now.duration_since(self.prev_frame_time).as_millis() as f32;
        self.ms_so_far += delta;
        self.prev_frame_time = now;
        if self.ms_so_far < self.min_ms_per_frame * self.frames_so_far as f32 {
            std::thread::sleep(std::time::Duration::from_millis((self.min_ms_per_frame * self.frames_so_far as f32 - self.ms_so_far) as u64));
        }
    }

    /// Sets the minimum window size
    pub fn set_min_window_size(&mut self, size: gfx::FloatSize) -> Result<()> {
        self.sdl_window.set_minimum_size(size.0 as u32, size.1 as u32).map_err(|e| anyhow!(e))
    }

    /// Sets key state
    fn set_key_state(&mut self, key: gfx::Key, state: bool) {
        *self.key_states.entry(key).or_insert(false) = state;
    }

    /// Blurs given texture
    pub(super) fn blur_region(&self, rect: gfx::Rect, radius: i32, gl_texture: u32, back_texture: u32, size: gfx::FloatSize, texture_transform: &Transformation) {
        self.blur_context.blur_region(rect, radius, gl_texture, back_texture, size, texture_transform);
        unsafe {
            gl::UseProgram(self.passthrough_shader.passthrough_shader);
        }
    }

    /// Blurs a given rectangle on the screen
    pub(super) fn blur_rect(&self, rect: gfx::Rect, radius: i32) {
        self.blur_region(rect, radius, self.window_texture, self.window_texture_back, self.get_window_size(), &self.normalization_transform);
    }

    pub const fn enable_blur(&mut self, enable: bool) {
        self.blur_context.blur_enabled = enable;
    }

    pub fn set_fps_limit(&mut self, fps: f32) {
        self.min_ms_per_frame = 1000.0 / fps;
        self.frames_so_far = 0;
        self.ms_so_far = 0.0;
    }

    pub const fn disable_fps_limit(&mut self) {
        self.min_ms_per_frame = 0.0;
    }

    pub fn enable_vsync(&self, enable: bool) {
        let swap_interval = if enable { SwapInterval::VSync } else { SwapInterval::Immediate };
        if let Err(error) = self.video_subsystem.gl_set_swap_interval(swap_interval) {
            println!("Error setting VSync: {error}");
        }
    }
}

/// The window, pointer, keyboard and clipboard, which is everything a UI element is allowed
/// to observe. Kept deliberately separate from rendering - see `gfx::UiContext`.
impl UiContext for GraphicsContext {
    fn get_window_size(&self) -> gfx::FloatSize {
        gfx::FloatSize(self.sdl_window.size().0 as f32 / self.real_scale, self.sdl_window.size().1 as f32 / self.real_scale)
    }

    fn get_mouse_pos(&self) -> gfx::FloatPos {
        gfx::FloatPos(
            self.sdl_event_pump.mouse_state().x() as f32 / self.real_scale,
            self.sdl_event_pump.mouse_state().y() as f32 / self.real_scale,
        )
    }

    fn get_key_state(&self, key: gfx::Key) -> bool {
        !self.block_key_states && *self.key_states.get(&key).unwrap_or(&false)
    }

    fn get_clipboard_text(&mut self) -> Option<String> {
        self.clipboard_context.get_text().ok()
    }

    fn set_clipboard_text(&mut self, text: &str) {
        if let Err(error) = self.clipboard_context.set_text(text.to_owned()) {
            println!("Error setting clipboard contents: {error}");
        }
    }

    fn as_graphics_context(&mut self) -> Option<&mut Self> {
        Some(self)
    }
}

/// Implement the Drop trait for the Renderer.
impl Drop for GraphicsContext {
    /// Closes, destroys the window and cleans up the resources.
    fn drop(&mut self) {
        unsafe {
            gl::DeleteFramebuffers(1, &raw const self.window_framebuffer);
            gl::DeleteTextures(1, &raw const self.window_texture);
            gl::DeleteTextures(1, &raw const self.window_texture_back);
        }
    }
}
