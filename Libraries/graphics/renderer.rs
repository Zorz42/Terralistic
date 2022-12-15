extern crate glfw;

use std::collections::HashMap;
use std::ffi::CString;
use self::glfw::{Context};
use crate::{BlendMode, Event, Key, Rect, set_blend_mode};
use crate::blur::BlurContext;
use crate::vertex_buffer_impl;
use crate::color;
use crate::events::{glfw_event_to_gfx_event, glfw_mouse_button_to_gfx_key};
use crate::shaders::compile_shader;
use crate::transformation::Transformation;
use crate::vertex_buffer_impl::VertexBufferImpl;

const VERTEX_SHADER_CODE: &str = r#"
#version 330 core

layout (location = 0) in vec2 vertex_position;
layout (location = 1) in vec4 vertex_color;
layout (location = 2) in vec2 vertex_texture_coordinate;

out vec4 fragment_color;
out vec2 texture_coord;

uniform int has_color_buffer;
uniform vec4 global_color;
uniform mat3 transform_matrix;
uniform mat3 texture_transform_matrix;

void main() {
	gl_Position = vec4(transform_matrix * vec3(vertex_position, 1), 1);
	fragment_color = global_color * vertex_color;
	texture_coord = (texture_transform_matrix * vec3(vertex_texture_coordinate.xy, 1)).xy;
}
"#;

const FRAGMENT_SHADER_CODE: &str = r#"
#version 330 core

in vec4 fragment_color;
in vec2 texture_coord;
layout(location = 0) out vec4 color;
uniform sampler2D texture_sampler;
uniform int has_texture;

void main() {
	color = mix(vec4(1.f, 1.f, 1.f, 1.f), texture(texture_sampler, texture_coord).rgba, has_texture) * fragment_color;
}
"#;

/**
This stores all shader uniform handles
 */
pub(crate) struct ShaderUniformHandles {
    pub has_texture: i32,
    pub global_color: i32,
    pub texture_sampler: i32,
    pub transform_matrix: i32,
    pub texture_transform_matrix: i32,
}

/**
This stores all the values needed for rendering.
 */
pub struct Renderer {
    pub(crate) uniforms: ShaderUniformHandles,
    pub(crate) glfw: glfw::Glfw,
    pub(crate) glfw_window: glfw::Window,
    pub(crate) glfw_events: std::sync::mpsc::Receiver<(f64, glfw::WindowEvent)>,
    pub(crate) default_shader: u32,
    pub(crate) normalization_transform: Transformation,
    pub(crate) rect_vertex_buffer: VertexBufferImpl,
    pub(crate) rect_outline_vertex_buffer: VertexBufferImpl,
    pub(crate) window_texture: u32,
    pub(crate) window_texture_back: u32,
    pub(crate) window_framebuffer: u32,
    pub(crate) blur_context: BlurContext,
    // Keep track of all Key states as a hashmap
    pub(crate) key_states: HashMap<Key, bool>,
    pub(crate) events: Vec<Event>,
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

        let mut result = Renderer {
            uniforms: ShaderUniformHandles {
                has_texture: 0,
                global_color: 0,
                texture_sampler: 0,
                transform_matrix: 0,
                texture_transform_matrix: 0,
            },
            glfw,
            glfw_window,
            glfw_events,
            default_shader: 0,
            normalization_transform: Transformation::new(),
            rect_vertex_buffer: VertexBufferImpl::new(),
            rect_outline_vertex_buffer: VertexBufferImpl::new(),
            window_texture: 0,
            window_texture_back: 0,
            window_framebuffer: 0,
            key_states: HashMap::new(),
            events: Vec::new(),
            blur_context: BlurContext::new(),
        };

        unsafe {
            gl::Enable(gl::BLEND);
        }
        set_blend_mode(BlendMode::Alpha);

        result.default_shader = compile_shader(VERTEX_SHADER_CODE, FRAGMENT_SHADER_CODE);

        unsafe {
            gl::UseProgram(result.default_shader);

            let ident = CString::new("has_texture").unwrap();
            result.uniforms.has_texture = gl::GetUniformLocation(result.default_shader, ident.as_ptr());
            let ident = CString::new("global_color").unwrap();
            result.uniforms.global_color = gl::GetUniformLocation(result.default_shader, ident.as_ptr());
            let ident = CString::new("texture_sampler").unwrap();
            result.uniforms.texture_sampler = gl::GetUniformLocation(result.default_shader, ident.as_ptr());
            let ident = CString::new("transform_matrix").unwrap();
            result.uniforms.transform_matrix = gl::GetUniformLocation(result.default_shader, ident.as_ptr());
            let ident = CString::new("texture_transform_matrix").unwrap();
            result.uniforms.texture_transform_matrix = gl::GetUniformLocation(result.default_shader, ident.as_ptr());

            gl::GenTextures(1, &mut result.window_texture);
            gl::GenTextures(1, &mut result.window_texture_back);
            gl::GenFramebuffers(1, &mut result.window_framebuffer);
            gl::BindFramebuffer(gl::FRAMEBUFFER, result.window_framebuffer);
        }

        result.rect_vertex_buffer.add_vertex(&vertex_buffer_impl::VertexImpl { x: 0.0, y: 0.0, color: color::Color { r: 255, g: 255, b: 255, a: 255 }, tex_x: 0.0, tex_y: 0.0 });
        result.rect_vertex_buffer.add_vertex(&vertex_buffer_impl::VertexImpl { x: 1.0, y: 0.0, color: color::Color { r: 255, g: 255, b: 255, a: 255 }, tex_x: 1.0, tex_y: 0.0 });
        result.rect_vertex_buffer.add_vertex(&vertex_buffer_impl::VertexImpl { x: 0.0, y: 1.0, color: color::Color { r: 255, g: 255, b: 255, a: 255 }, tex_x: 0.0, tex_y: 1.0 });

        result.rect_vertex_buffer.add_vertex(&vertex_buffer_impl::VertexImpl { x: 1.0, y: 1.0, color: color::Color { r: 255, g: 255, b: 255, a: 255 }, tex_x: 1.0, tex_y: 1.0 });
        result.rect_vertex_buffer.add_vertex(&vertex_buffer_impl::VertexImpl { x: 1.0, y: 0.0, color: color::Color { r: 255, g: 255, b: 255, a: 255 }, tex_x: 1.0, tex_y: 0.0 });
        result.rect_vertex_buffer.add_vertex(&vertex_buffer_impl::VertexImpl { x: 0.0, y: 1.0, color: color::Color { r: 255, g: 255, b: 255, a: 255 }, tex_x: 0.0, tex_y: 1.0 });

        result.rect_vertex_buffer.upload();

        result.rect_outline_vertex_buffer.add_vertex(&vertex_buffer_impl::VertexImpl { x: 0.0, y: 0.0, color: color::Color { r: 255, g: 255, b: 255, a: 255 }, tex_x: 0.0, tex_y: 0.0 });
        result.rect_outline_vertex_buffer.add_vertex(&vertex_buffer_impl::VertexImpl { x: 1.0, y: 0.0, color: color::Color { r: 255, g: 255, b: 255, a: 255 }, tex_x: 1.0, tex_y: 0.0 });

        result.rect_outline_vertex_buffer.add_vertex(&vertex_buffer_impl::VertexImpl { x: 1.0, y: 0.0, color: color::Color { r: 255, g: 255, b: 255, a: 255 }, tex_x: 1.0, tex_y: 0.0 });
        result.rect_outline_vertex_buffer.add_vertex(&vertex_buffer_impl::VertexImpl { x: 1.0, y: 1.0, color: color::Color { r: 255, g: 255, b: 255, a: 255 }, tex_x: 1.0, tex_y: 1.0 });

        result.rect_outline_vertex_buffer.add_vertex(&vertex_buffer_impl::VertexImpl { x: 1.0, y: 1.0, color: color::Color { r: 255, g: 255, b: 255, a: 255 }, tex_x: 1.0, tex_y: 1.0 });
        result.rect_outline_vertex_buffer.add_vertex(&vertex_buffer_impl::VertexImpl { x: 0.0, y: 1.0, color: color::Color { r: 255, g: 255, b: 255, a: 255 }, tex_x: 0.0, tex_y: 1.0 });

        result.rect_outline_vertex_buffer.add_vertex(&vertex_buffer_impl::VertexImpl { x: 0.0, y: 1.0, color: color::Color { r: 255, g: 255, b: 255, a: 255 }, tex_x: 0.0, tex_y: 1.0 });
        result.rect_outline_vertex_buffer.add_vertex(&vertex_buffer_impl::VertexImpl { x: 0.0, y: 0.0, color: color::Color { r: 255, g: 255, b: 255, a: 255 }, tex_x: 0.0, tex_y: 0.0 });

        result.rect_outline_vertex_buffer.upload();

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
        }
    }

    /**
    A function that checks mouse buttons
    and sends events to the event handler
    if they change state.
     */
    pub fn handle_mouse_buttons(&mut self) {
        let mouse_buttons = [
            glfw::MouseButton::Button1,
            glfw::MouseButton::Button2,
            glfw::MouseButton::Button3,
            glfw::MouseButton::Button4,
            glfw::MouseButton::Button5,
            glfw::MouseButton::Button6,
            glfw::MouseButton::Button7,
            glfw::MouseButton::Button8,
        ];

        for glfw_button in mouse_buttons {
            let glfw_state = self.glfw_window.get_mouse_button(glfw_button);
            let button = glfw_mouse_button_to_gfx_key(glfw_button);

            let state = match glfw_state {
                glfw::Action::Press => true,
                glfw::Action::Release => false,
                _ => continue,
            };

            if self.get_key_state(button) != state {
                self.set_key_state(button, state);

                if state {
                    self.events.push(Event::KeyPress(button));
                } else {
                    self.events.push(Event::KeyRelease(button));
                }
            }
        }
    }

    /**
    Returns an array of events, such as key presses
     */
    pub fn get_events(&mut self) -> Vec<Event> {
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
    pub fn post_render(&mut self) {
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

        self.handle_mouse_buttons();
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
            gl::UseProgram(self.default_shader);
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