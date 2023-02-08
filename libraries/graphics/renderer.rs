use super::blur::BlurContext;
use super::events::sdl_event_to_gfx_event;
use super::passthrough_shader::PassthroughShader;
use super::shadow::ShadowContext;
use super::transformation::Transformation;
use super::{set_blend_mode, BlendMode, Event, Key, Rect};
use copypasta::ClipboardContext;
use std::collections::{HashMap, VecDeque};

/**
This stores all the values needed for rendering.
 */
pub struct Renderer {
    _gl_context: sdl2::video::GLContext,
    sdl_window: sdl2::video::Window,
    sdl_event_pump: sdl2::EventPump,
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
}

impl Renderer {
    /**
    Initializes all the values needed for rendering.
     */
    pub fn new(window_width: i32, window_height: i32, window_title: &str) -> Self {
        assert!(
            !(window_width <= 0 || window_height <= 0),
            "Invalid window dimensions"
        );

        let sdl = sdl2::init().unwrap();
        let video_subsystem = sdl.video().unwrap();

        let gl_attr = video_subsystem.gl_attr();

        gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
        gl_attr.set_context_version(3, 3);

        let sdl_window = video_subsystem
            .window(window_title, window_width as u32, window_height as u32)
            .position_centered()
            .opengl()
            .resizable()
            .build()
            .unwrap();

        let gl_context = sdl_window.gl_create_context().unwrap();
        gl::load_with(|s| {
            video_subsystem
                .gl_get_proc_address(s)
                .cast::<std::ffi::c_void>()
        });

        unsafe {
            gl::Enable(gl::BLEND);
        }
        set_blend_mode(BlendMode::Alpha);

        // enable vsync
        video_subsystem.gl_set_swap_interval(1).unwrap();

        let passthrough_shader = PassthroughShader::new();
        let mut window_texture = 0;
        let mut window_texture_back = 0;
        let mut window_framebuffer = 0;

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
            sdl_event_pump: sdl.event_pump().unwrap(),
            normalization_transform: Transformation::new(),
            window_texture,
            window_texture_back,
            window_framebuffer,
            key_states: HashMap::new(),
            events: Vec::new(),
            blur_context: BlurContext::new().unwrap(),
            passthrough_shader,
            shadow_context,
            events_queue: VecDeque::new(),
            window_open: true,
            clipboard_context: ClipboardContext::new().unwrap(),
        };

        result.handle_window_resize();

        result
    }

    /**
    Is called every time the window is resized.
     */
    pub fn handle_window_resize(&mut self) {
        let (width, height) = self.sdl_window.size();
        let width = width as i32;
        let height = height as i32;

        self.normalization_transform = Transformation::new();
        self.normalization_transform
            .stretch(2.0 / width as f32, -2.0 / height as f32);
        self.normalization_transform
            .translate(-width as f32 / 2.0, -height as f32 / 2.0);

        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.window_texture);
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA as gl::types::GLint,
                self.get_window_width() as i32,
                self.get_window_height() as i32,
                0,
                gl::BGRA as gl::types::GLenum,
                gl::UNSIGNED_BYTE,
                std::ptr::null(),
            );

            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_MAG_FILTER,
                gl::NEAREST as gl::types::GLint,
            );
            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_MIN_FILTER,
                gl::NEAREST as gl::types::GLint,
            );

            gl::BindTexture(gl::TEXTURE_2D, self.window_texture_back);
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA as gl::types::GLint,
                self.get_window_width() as i32,
                self.get_window_height() as i32,
                0,
                gl::BGRA,
                gl::UNSIGNED_BYTE,
                std::ptr::null(),
            );

            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_MAG_FILTER,
                gl::NEAREST as gl::types::GLint,
            );
            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_MIN_FILTER,
                gl::NEAREST as gl::types::GLint,
            );

            // set glViewport
            gl::Viewport(
                0,
                0,
                self.get_window_width() as i32,
                self.get_window_height() as i32,
            );
        }
    }

    /**
    Returns an array of deprecated_events, such as key presses
     */
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

    /**
    Returns an event, returns None if there are no deprecated_events
     */
    pub fn get_event(&mut self) -> Option<Event> {
        for event in self.get_events() {
            self.events_queue.push_back(event);
        }

        if self.events_queue.is_empty() {
            None
        } else {
            Some(self.events_queue.pop_front().unwrap())
        }
    }

    /**
    Checks if the window is open, this becomes false, when the user closes the window, or the program closes it
     */
    pub fn is_window_open(&self) -> bool {
        self.window_open
    }

    /**
    Closes the window
     */
    pub fn close_window(&mut self) {
        self.window_open = false;
    }

    /**
    Should be called after rendering
     */
    pub fn update_window(&mut self) {
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
                    self.get_window_width() as i32 * 2,
                    self.get_window_height() as i32 * 2,
                    0,
                    0,
                    self.get_window_width() as i32 * 2,
                    self.get_window_height() as i32 * 2,
                    gl::COLOR_BUFFER_BIT,
                    gl::NEAREST,
                );
            }
            #[cfg(target_os = "macos")]
            {
                gl::BlitFramebuffer(
                    0,
                    0,
                    (self.get_window_width() as f32 * 2.0) as i32,
                    (self.get_window_height() as f32 * 2.0) as i32,
                    0,
                    0,
                    self.get_window_width() as i32 * 2,
                    self.get_window_height() as i32 * 2,
                    gl::COLOR_BUFFER_BIT,
                    gl::NEAREST,
                );
            }
            #[cfg(target_os = "linux")]
            {
                gl::BlitFramebuffer(
                    0,
                    0,
                    (self.get_window_width() as f32 * 2.0) as i32,
                    (self.get_window_height() as f32 * 2.0) as i32,
                    0,
                    0,
                    self.get_window_width() as i32 * 2,
                    self.get_window_height() as i32 * 2,
                    gl::COLOR_BUFFER_BIT,
                    gl::NEAREST,
                );
            }
        }

        self.sdl_window.gl_swap_window();

        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, self.window_framebuffer);
        }
    }

    /**
    Sets the minimum window size
     */
    pub fn set_min_window_size(&mut self, width: u32, height: u32) {
        self.sdl_window
            .set_minimum_size(width, height)
            .expect("Failed to set minimum window size");
    }

    /**
    Get the current window width
     */
    pub fn get_window_width(&self) -> u32 {
        self.sdl_window.size().0
    }

    /**
    Get the current window height
     */
    pub fn get_window_height(&self) -> u32 {
        self.sdl_window.size().1
    }

    /**
    Gets mouse x position
     */
    pub fn get_mouse_x(&self) -> f32 {
        // get mouse position
        self.sdl_event_pump.mouse_state().x() as f32
    }

    /**
    Gets mouse y position
     */
    pub fn get_mouse_y(&self) -> f32 {
        self.sdl_event_pump.mouse_state().y() as f32
    }

    /**
    Gets key state
     */
    pub fn get_key_state(&self, key: Key) -> bool {
        *self.key_states.get(&key).unwrap_or(&false)
    }

    /**
    Sets key state
     */
    pub fn set_key_state(&mut self, key: Key, state: bool) {
        *self.key_states.entry(key).or_insert(false) = state;
    }

    /**
    Blurs given texture
     */
    pub(super) fn blur_region(
        &self,
        rect: Rect,
        radius: i32,
        gl_texture: u32,
        back_texture: u32,
        size: (f32, f32),
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
        unsafe {
            gl::UseProgram(self.passthrough_shader.passthrough_shader);
        }
    }

    /**
    Blurs a given rectangle on the screen
     */
    pub(super) fn blur_rect(&self, rect: Rect, radius: i32) {
        self.blur_region(
            rect,
            radius,
            self.window_texture,
            self.window_texture_back,
            (
                self.get_window_width() as f32,
                self.get_window_height() as f32,
            ),
            &self.normalization_transform,
        );
    }
}

/**
Implement the Drop trait for the Renderer
 */
impl Drop for Renderer {
    /**
    Closes, destroys the window and cleans up the resources
     */
    fn drop(&mut self) {
        unsafe {
            gl::DeleteFramebuffers(1, &self.window_framebuffer);
            gl::DeleteTextures(1, &self.window_texture);
            gl::DeleteTextures(1, &self.window_texture_back);
        }
    }
}
