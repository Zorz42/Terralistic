extern crate glfw;
extern crate queues;

use queues::*;
use std::collections::HashMap;
use std::ffi::CString;
use self::glfw::{Context};
use crate::{BlendMode, Event, Key, Rect, set_blend_mode};
use crate::blur::BlurContext;
use crate::vertex_buffer_impl;
use crate::color;
use crate::events::{glfw_event_to_gfx_event, glfw_mouse_button_to_gfx_key};
use crate::passthrough_shader::PassthroughShader;
use crate::shaders::compile_shader;
use crate::shadow::ShadowContext;
use crate::transformation::Transformation;
use crate::vertex_buffer_impl::VertexBufferImpl;

/**
This stores all the values needed for rendering.
 */
pub struct Renderer {
    pub(crate) glfw: glfw::Glfw,
    pub(crate) glfw_window: glfw::Window,
    pub(crate) glfw_events: std::sync::mpsc::Receiver<(f64, glfw::WindowEvent)>,
    pub(crate) normalization_transform: Transformation,
    pub(crate) window_texture: u32,
    pub(crate) window_texture_back: u32,
    pub(crate) window_framebuffer: u32,
    pub(crate) blur_context: BlurContext,
    pub(crate) passthrough_shader: PassthroughShader,
    pub(crate) events_queue: Queue<Event>,
    // Keep track of all Key states as a hashmap
    pub(crate) key_states: HashMap<Key, bool>,
    pub(crate) events: Vec<Event>,
    pub(crate) shadow_context: ShadowContext,
}

impl Renderer {
    /**
    Initializes all the values needed for rendering.
     */
    pub fn new(window_width: i32, window_height: i32, window_title: String) -> Self {
        if window_width <= 0 || window_height <= 0 {
            panic!("Invalid window dimensions");
        }

        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

        glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
        #[cfg(target_os = "macos")]
        glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));

        let (mut glfw_window, glfw_events) = glfw.create_window(window_width as u32, window_height as u32, window_title.as_str(), glfw::WindowMode::Windowed).expect("Failed to create GLFW window.");

        glfw_window.make_current();
        glfw_window.set_key_polling(true);
        glfw_window.set_framebuffer_size_polling(true);

        gl::load_with(|symbol| glfw_window.get_proc_address(symbol) as *const _);

        unsafe {
            gl::Enable(gl::BLEND);
        }
        set_blend_mode(BlendMode::Alpha);

        let passthrough_shader = PassthroughShader::new();
        let mut window_texture= 0;
        let mut window_texture_back= 0;
        let mut window_framebuffer= 0;

        glfw_window.set_scroll_polling(true);
        glfw_window.set_mouse_button_polling(true);

        unsafe {
            gl::GenTextures(1, &mut window_texture);
            gl::GenTextures(1, &mut window_texture_back);
            gl::GenFramebuffers(1, &mut window_framebuffer);
            gl::BindFramebuffer(gl::FRAMEBUFFER, window_framebuffer);
        }

        let shadow_context = ShadowContext::new();

        let mut result = Renderer {
            glfw,
            glfw_window,
            glfw_events,
            normalization_transform: Transformation::new(),
            window_texture,
            window_texture_back,
            window_framebuffer,
            key_states: HashMap::new(),
            events: Vec::new(),
            blur_context: BlurContext::new(),
            passthrough_shader,
            shadow_context,
            events_queue: Queue::new(),
        };

        result.handle_window_resize();

        result
    }

    /**
    Is called every time the window is resized.
     */
    pub fn handle_window_resize(&mut self) {
        let (width, height) = self.glfw_window.get_size();

        self.normalization_transform = Transformation::new();
        self.normalization_transform.stretch(2.0 / width as f32, -2.0 / height as f32);
        self.normalization_transform.translate(-width as f32 / 2.0, -height as f32 / 2.0);

        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.window_texture);
            gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGBA as gl::types::GLint, width * 2, height * 2, 0, gl::BGRA as gl::types::GLenum, gl::UNSIGNED_BYTE, std::ptr::null());

            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as gl::types::GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as gl::types::GLint);

            gl::BindTexture(gl::TEXTURE_2D, self.window_texture_back);
            gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGBA as gl::types::GLint, width * 2, height * 2, 0, gl::BGRA, gl::UNSIGNED_BYTE, std::ptr::null());

            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as gl::types::GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as gl::types::GLint);

            // set glViewport
            gl::Viewport(0, 0, width * 2, height * 2);
        }
    }

    /**
    Returns an array of events, such as key presses
     */
    fn get_events(&mut self) -> Vec<Event> {
        let mut glfw_events = vec![];

        for (_, glfw_event) in glfw::flush_messages(&self.glfw_events) {
            glfw_events.push(glfw_event);
        }

        for glfw_event in glfw_events {
            match glfw_event {
                glfw::WindowEvent::FramebufferSize(_width, _height) => {
                    self.handle_window_resize();
                }
                _ => {}
            }

            if let Some(event) = glfw_event_to_gfx_event(glfw_event) {
                // if event is a key press event update the key states to true
                if let Event::KeyPress(key) = event {
                    self.set_key_state(key, true);
                }
                // if event is a key release event update the key states to false
                if let Event::KeyRelease(key) = event {
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
    Returns an event, returns None if there are no events
     */
    pub fn get_event(&mut self) -> Option<Event> {
        for event in self.get_events() {
            self.events_queue.add(event).unwrap();
        }

        if self.events_queue.size() == 0 {
            None
        } else {
            Some(self.events_queue.remove().unwrap())
        }
    }

    /**
    Checks if the window is open, this becomes false, when the user closes the window, or the program closes it
     */
    pub fn is_window_open(&self) -> bool {
        !self.glfw_window.should_close()
    }

    /**
    Closes the window
     */
    pub fn close_window(&mut self) {
        self.glfw_window.set_should_close(true);
    }

    /**
    Should be called after rendering
     */
    pub fn update_window(&mut self) {
        unsafe {
            gl::BindFramebuffer(gl::READ_FRAMEBUFFER, self.window_framebuffer);
            gl::FramebufferTexture2D(gl::READ_FRAMEBUFFER, gl::COLOR_ATTACHMENT0, gl::TEXTURE_2D, self.window_texture, 0);
            gl::BindFramebuffer(gl::DRAW_FRAMEBUFFER, 0);

            gl::BlitFramebuffer(0, 0, self.glfw_window.get_size().0 * 2, self.glfw_window.get_size().1 * 2, 0, 0, self.glfw_window.get_size().0 * 2, self.glfw_window.get_size().1 * 2, gl::COLOR_BUFFER_BIT, gl::NEAREST);
        }

        self.glfw_window.swap_buffers();
        self.glfw.poll_events();

        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, self.window_framebuffer);
        }
    }

    /**
    Sets the minimum window size
     */
    pub fn set_min_window_size(&mut self, width: u32, height: u32) {
        self.glfw_window.set_size_limits(Some(width), Some(height), None, None);
    }

    /**
    Get the current window width
     */
    pub fn get_window_width(&self) -> u32 {
        self.glfw_window.get_size().0.try_into().unwrap()
    }

    /**
    Get the current window height
     */
    pub fn get_window_height(&self) -> u32 {
        self.glfw_window.get_size().1.try_into().unwrap()
    }

    /**
    Gets mouse x position
     */
    pub fn get_mouse_x(&self) -> f32 {
        self.glfw_window.get_cursor_pos().0 as f32
    }

    /**
    Gets mouse y position
     */
    pub fn get_mouse_y(&self) -> f32 {
        self.glfw_window.get_cursor_pos().1 as f32
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
    pub(crate) fn blur_region(&self, rect: &Rect, radius: i32, gl_texture: u32, back_texture: u32, width: f32, height: f32, texture_transform: &Transformation) {
        self.blur_context.blur_region(rect, radius, gl_texture, back_texture, width, height, texture_transform);
        unsafe {
            gl::UseProgram(self.passthrough_shader.passthrough_shader);
        }
    }

    /**
    Blurs a given rectangle on the screen
     */
    pub(crate) fn blur_rect(&self, rect: &Rect, radius: i32) {
        self.blur_region(rect, radius, self.window_texture, self.window_texture_back, self.glfw_window.get_size().0 as f32, self.glfw_window.get_size().1 as f32, &self.normalization_transform);
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
        // close glfw window
        self.glfw_window.set_should_close(true);
    }
}