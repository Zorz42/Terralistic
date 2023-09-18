extern crate alloc;

use alloc::collections::VecDeque;
use std::collections::HashMap;

use anyhow::{anyhow, Result};
use copypasta::ClipboardContext;
use sdl2::video::SwapInterval;

use crate::libraries::graphics as gfx;
use crate::libraries::graphics::events::sdl_event_to_gfx_event;

use super::blur::BlurContext;
use super::passthrough_shader::PassthroughShader;
use super::shadow::ShadowContext;
use super::transformation::Transformation;
use super::{set_blend_mode, BlendMode, Event, Key, Rect};

/// This stores all the values needed for rendering.
pub struct Renderer {
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
    events_queue: VecDeque<Event>,
    window_open: bool,
    // Keep track of all Key states as a hashmap
    key_states: HashMap<Key, bool>,
    events: Vec<Event>,
    pub(super) shadow_context: ShadowContext,
    pub clipboard_context: ClipboardContext,
    pub block_key_states: bool,
    pub scale: f32,
    real_scale: f32,
    min_ms_per_frame: f32,
    frames_so_far: u32,
    ms_so_far: f32,
    prev_frame_time: std::time::Instant,
}

impl Renderer {
    /// Initializes all the values needed for rendering.
    /// It usually fails because the system doesn't support graphics.
    /// # Errors
    /// - This function can fail if SDL2 fails to initialize.
    /// - This function can fail if OpenGL fails to initialize.
    /// - This function can fail if the window fails to initialize.
    /// - This function can fail if the passthrough shader fails to initialize.
    /// - This function can fail if the blur context fails to initialize.
    /// - This function can fail if the shadow context fails to initialize.
    /// - This function can fail if the clipboard context fails to initialize.
    pub fn new(window_width: u32, window_height: u32, window_title: &str) -> Result<Self> {
        let sdl = sdl2::init();
        let sdl = sdl.map_err(|e| anyhow!(e))?;
        let video_subsystem = sdl.video();
        let video_subsystem = video_subsystem.map_err(|e| anyhow!(e))?;

        let gl_attr = video_subsystem.gl_attr();

        gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
        gl_attr.set_context_version(3, 3);

        let sdl_window = video_subsystem
            .window(window_title, window_width, window_height)
            .position_centered()
            .opengl()
            .resizable()
            .build()?;

        let gl_context = sdl_window.gl_create_context().map_err(|e| anyhow!(e))?;
        gl::load_with(|s| {
            video_subsystem
                .gl_get_proc_address(s)
                .cast::<core::ffi::c_void>()
        });

        // Safety: We are calling OpenGL functions safely.
        unsafe {
            gl::Enable(gl::BLEND);
        }
        set_blend_mode(BlendMode::Alpha);

        let passthrough_shader = PassthroughShader::new()?;
        let mut window_texture = 0;
        let mut window_texture_back = 0;
        let mut window_framebuffer = 0;

        // Safety: We are calling OpenGL functions safely.
        unsafe {
            gl::GenTextures(1, &mut window_texture);
            gl::GenTextures(1, &mut window_texture_back);
            gl::GenFramebuffers(1, &mut window_framebuffer);
            gl::BindFramebuffer(gl::FRAMEBUFFER, window_framebuffer);
        }

        let shadow_context = ShadowContext::new();

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
            clipboard_context: ClipboardContext::new().map_err(|e| anyhow!(e))?,
            block_key_states: false,
            scale: 1.0,
            real_scale: 1.0,
            min_ms_per_frame: 0.0,
            frames_so_far: 0,
            ms_so_far: 0.0,
            prev_frame_time: std::time::Instant::now(),
        };

        result.handle_window_resize();

        Ok(result)
    }

    /// Is called every time the window is resized.
    pub fn handle_window_resize(&mut self) {
        // Safety: We are calling OpenGL functions safely.
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
                core::ptr::null(),
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
                core::ptr::null(),
            );

            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);

            gl::Viewport(
                0,
                0,
                self.sdl_window.size().0 as i32,
                self.sdl_window.size().1 as i32,
            );
        }
    }

    /// Returns an array of events, such as key presses.
    fn get_events(&mut self) -> Vec<Event> {
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
                if let Event::KeyPress(key, ..) = event {
                    self.set_key_state(key, true);
                }
                // if event is a key release event update the key states to false
                if let Event::KeyRelease(key, ..) = event {
                    self.set_key_state(key, false);
                }

                self.events.push(event);
            }
        }

        let result = self.events.clone();
        self.events.clear();

        result
    }

    /// Returns an event, returns None if there are no events
    pub fn get_event(&mut self) -> Option<Event> {
        for event in self.get_events() {
            self.events_queue.push_back(event);
        }
        self.events_queue.pop_front()
    }

    /// Checks if the window is open, this becomes false, when the user closes the window, or the program closes it
    pub const fn is_window_open(&self) -> bool {
        self.window_open
    }

    /// Closes the window
    pub fn close_window(&mut self) {
        self.window_open = false;
    }

    /// Should be called after rendering
    pub fn update_window(&mut self) {
        self.blur_context.update();

        self.real_scale += (self.scale - self.real_scale) / 5.0;
        if f32::abs(self.real_scale - self.scale) < 0.001 {
            self.real_scale = self.scale;
        }

        self.normalization_transform = Transformation::new();
        self.normalization_transform
            .translate(gfx::FloatPos(-1.0, 1.0));
        self.normalization_transform.stretch((
            2.0 / self.get_window_size().0,
            -2.0 / self.get_window_size().1,
        ));

        // Safety: We are calling OpenGL functions safely.
        unsafe {
            gl::BindFramebuffer(gl::READ_FRAMEBUFFER, self.window_framebuffer);
            gl::FramebufferTexture2D(
                gl::READ_FRAMEBUFFER,
                gl::COLOR_ATTACHMENT0,
                gl::TEXTURE_2D,
                self.window_texture,
                0,
            );
            gl::BindFramebuffer(gl::DRAW_FRAMEBUFFER, 0);

            #[cfg(target_os = "windows")]
            {
                gl::BlitFramebuffer(
                    0,
                    0,
                    (self.get_window_size().0 * 2.0) as i32,
                    (self.get_window_size().1 * 2.0) as i32,
                    0,
                    0,
                    (self.get_window_size().0 * 2.0) as i32,
                    (self.get_window_size().1 * 2.0) as i32,
                    gl::COLOR_BUFFER_BIT,
                    gl::NEAREST,
                );
            }
            #[cfg(target_os = "macos")]
            {
                gl::BlitFramebuffer(
                    0,
                    0,
                    (self.get_window_size().0 * 2.0) as i32,
                    (self.get_window_size().1 * 2.0) as i32,
                    0,
                    0,
                    (self.get_window_size().0 * 2.0) as i32,
                    (self.get_window_size().1 * 2.0) as i32,
                    gl::COLOR_BUFFER_BIT,
                    gl::NEAREST,
                );
            }
            #[cfg(target_os = "linux")]
            {
                gl::BlitFramebuffer(
                    0,
                    0,
                    (self.get_window_size().0 as f32 * 2.0) as i32,
                    (self.get_window_size().1 as f32 * 2.0) as i32,
                    0,
                    0,
                    (self.get_window_size().0 as f32 * 2.0) as i32,
                    (self.get_window_size().1 as f32 * 2.0) as i32,
                    gl::COLOR_BUFFER_BIT,
                    gl::NEAREST,
                );
            }
        }

        self.sdl_window.gl_swap_window();

        // Safety: We are calling OpenGL functions safely.
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, self.window_framebuffer);
        }

        self.frames_so_far += 1;
        let now = std::time::Instant::now();
        let delta = now.duration_since(self.prev_frame_time).as_millis() as f32;
        self.ms_so_far += delta;
        self.prev_frame_time = now;
        if self.ms_so_far < self.min_ms_per_frame * self.frames_so_far as f32 {
            std::thread::sleep(core::time::Duration::from_millis(
                (self.min_ms_per_frame * self.frames_so_far as f32 - self.ms_so_far) as u64,
            ));
        }
    }

    /// Sets the minimum window size
    pub fn set_min_window_size(&mut self, size: gfx::FloatSize) -> Result<()> {
        self.sdl_window
            .set_minimum_size(size.0 as u32, size.1 as u32)
            .map_err(|e| anyhow!(e))
    }

    /// Get the current window size
    pub fn get_window_size(&self) -> gfx::FloatSize {
        gfx::FloatSize(
            self.sdl_window.size().0 as f32 / self.real_scale,
            self.sdl_window.size().1 as f32 / self.real_scale,
        )
    }

    /// Gets mouse position
    pub fn get_mouse_pos(&self) -> gfx::FloatPos {
        gfx::FloatPos(
            self.sdl_event_pump.mouse_state().x() as f32 / self.real_scale,
            self.sdl_event_pump.mouse_state().y() as f32 / self.real_scale,
        )
    }

    /// Gets key state
    pub fn get_key_state(&self, key: Key) -> bool {
        !self.block_key_states && *self.key_states.get(&key).unwrap_or(&false)
    }

    /// Sets key state
    fn set_key_state(&mut self, key: Key, state: bool) {
        *self.key_states.entry(key).or_insert(false) = state;
    }

    /// Blurs given texture
    pub(super) fn blur_region(
        &self,
        rect: Rect,
        radius: i32,
        gl_texture: u32,
        back_texture: u32,
        size: gfx::FloatSize,
        texture_transform: &Transformation,
    ) {
        self.blur_context.blur_region(
            rect,
            radius,
            gl_texture,
            back_texture,
            size,
            texture_transform,
        );
        // Safety: We are calling OpenGL functions safely.
        unsafe {
            gl::UseProgram(self.passthrough_shader.passthrough_shader);
        }
    }

    /// Blurs a given rectangle on the screen
    pub(super) fn blur_rect(&self, rect: Rect, radius: i32) {
        self.blur_region(
            rect,
            radius,
            self.window_texture,
            self.window_texture_back,
            self.get_window_size(),
            &self.normalization_transform,
        );
    }

    pub fn enable_blur(&mut self, enable: bool) {
        self.blur_context.blur_enabled = enable;
    }

    pub fn set_fps_limit(&mut self, fps: f32) {
        self.min_ms_per_frame = 1000.0 / fps;
        self.frames_so_far = 0;
        self.ms_so_far = 0.0;
    }

    pub fn disable_fps_limit(&mut self) {
        self.min_ms_per_frame = 0.0;
    }

    pub fn enable_vsync(&mut self, enable: bool) {
        let swap_interval = if enable {
            SwapInterval::VSync
        } else {
            SwapInterval::Immediate
        };
        if let Err(error) = self.video_subsystem.gl_set_swap_interval(swap_interval) {
            println!("Error setting VSync: {error}");
        }
    }
}

/// Implement the Drop trait for the Renderer.
impl Drop for Renderer {
    /// Closes, destroys the window and cleans up the resources.
    fn drop(&mut self) {
        // Safety: We are calling OpenGL functions safely.
        unsafe {
            gl::DeleteFramebuffers(1, &self.window_framebuffer);
            gl::DeleteTextures(1, &self.window_texture);
            gl::DeleteTextures(1, &self.window_texture_back);
        }
    }
}
