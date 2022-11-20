extern crate glfw;
use self::glfw::{Context};
use crate::transformation;
use crate::vertex_buffer_impl;
use crate::vertex_buffer_impl::{VertexBufferImpl, VertexImpl};
use crate::color;

/*const VERTEX_SHADER_CODE: &str = r#"
#version 330 core
layout(location = 0) in vec2 vertex_position;
layout(location = 1) in vec4 vertex_color;
layout(location = 2) in vec2 vertex_uv;
out vec4 fragment_color;
out vec2 uv;
uniform int has_color_buffer;
uniform vec4 default_color;
uniform mat3 transform_matrix;
uniform mat3 texture_transform_matrix;
void main() {
   gl_Position = vec4(transform_matrix * vec3(vertex_position.xy, 1), 1);
   fragment_color = mix(default_color, vertex_color, has_color_buffer);
   uv = (texture_transform_matrix * vec3(vertex_uv, 1)).xy;
}
"#;

const FRAGMENT_SHADER_CODE: &str = r#"
#version 330 core
in vec4 fragment_color;
in vec2 uv;
layout(location = 0) out vec4 color;
uniform sampler2D texture_sampler;
uniform int has_texture;
void main() {
   color = mix(vec4(1.f, 1.f, 1.f, 1.f), texture(texture_sampler, uv).rgba, has_texture) * fragment_color;
}
"#;*/

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

/*
This stores all values needed for OpenGL and GLFW.
*/
pub struct GlfwContext {
    pub glfw: glfw::Glfw,
    pub glfw_window: glfw::Window,
    pub window_events: std::sync::mpsc::Receiver<(f64, glfw::WindowEvent)>,
}

pub enum BlendMode {
    Alpha,
    Multiply,
}

pub fn set_blend_mode(blend_mode: BlendMode) {
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

/*
Initialized OpenGL and GLFW. Returns the
context with initialized variables.
*/
pub fn init_glfw(window_width: i32, window_height: i32, window_title: String) -> GlfwContext {
    if window_width <= 0 || window_height <= 0 {
        panic!("Invalid window dimensions");
    }

    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
    #[cfg(target_os = "macos")]
    glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));

    let (mut glfw_window, window_events) = glfw.create_window(window_width as u32, window_height as u32, window_title.as_str(), glfw::WindowMode::Windowed).expect("Failed to create GLFW window.");

    glfw_window.make_current();
    glfw_window.set_key_polling(true);
    glfw_window.set_framebuffer_size_polling(true);

    gl::load_with(|symbol| glfw_window.get_proc_address(symbol) as *const _);

    unsafe {
        gl::Enable(gl::BLEND);
    }
    set_blend_mode(BlendMode::Alpha);

    GlfwContext{
        glfw,
        glfw_window,
        window_events,
    }
}

/*
This compiles fragment and vertex shader and returns the compiled shader program
*/
pub fn compile_shader(vertex_code: &str, fragment_code: &str) -> u32 {
    unsafe {
        let vertex_id = gl::CreateShader(gl::VERTEX_SHADER);
        let fragment_id = gl::CreateShader(gl::FRAGMENT_SHADER);

        let temp = std::ffi::CString::new(vertex_code).unwrap();
        gl::ShaderSource(vertex_id, 1, &temp.as_ptr(), std::ptr::null());
        gl::CompileShader(vertex_id);

        let temp = std::ffi::CString::new(fragment_code).unwrap();
        gl::ShaderSource(fragment_id, 1, &temp.as_ptr(), std::ptr::null());
        gl::CompileShader(fragment_id);

        let program_id = gl::CreateProgram();

        gl::AttachShader(program_id, vertex_id);
        gl::AttachShader(program_id, fragment_id);
        gl::LinkProgram(program_id);

        gl::DetachShader(program_id, vertex_id);
        gl::DetachShader(program_id, fragment_id);

        gl::DeleteShader(vertex_id);
        gl::DeleteShader(fragment_id);

        program_id
    }
}

/*
This stores all shader uniform handles
*/
pub(crate) struct ShaderUniformHandles {
    pub has_texture: i32,
    pub global_color: i32,
    pub texture_sampler: i32,
    pub transform_matrix: i32,
    pub texture_transform_matrix: i32,
}

/*
This stores all values needed for rendering.
*/
pub struct Renderer {
    pub(crate) uniforms: ShaderUniformHandles,
    pub(crate) glfw: glfw::Glfw,
    pub(crate) glfw_window: glfw::Window,
    pub(crate) default_shader: u32,
    pub(crate) normalization_transform: transformation::Transformation,
    pub(crate) rect_vertex_buffer: VertexBufferImpl,
}

impl Renderer {
    pub fn new(glfw: glfw::Glfw, glfw_window: glfw::Window) -> Self {
        Renderer{
            uniforms: ShaderUniformHandles {
                has_texture: 0,
                global_color: 0,
                texture_sampler: 0,
                transform_matrix: 0,
                texture_transform_matrix: 0,
            },
            glfw,
            glfw_window,
            default_shader: 0,
            normalization_transform: transformation::Transformation::new(),
            rect_vertex_buffer: vertex_buffer_impl::VertexBufferImpl::new(),
        }
    }

    /*
    Checks if the window is open, this becomes false, when the user closes the window, or the program closes it
    */
    pub fn is_window_open(&self) -> bool {
        !self.glfw_window.should_close()
    }

    /*
    Initialized renderer, sets some values and initializes them
    */
    pub fn init(&mut self) {
        self.default_shader = compile_shader(VERTEX_SHADER_CODE, FRAGMENT_SHADER_CODE);

        unsafe {
            gl::UseProgram(self.default_shader);

            let ident = std::ffi::CString::new("has_texture").unwrap();
            self.uniforms.has_texture = gl::GetUniformLocation(self.default_shader, ident.as_ptr());
            let ident = std::ffi::CString::new("global_color").unwrap();
            self.uniforms.global_color = gl::GetUniformLocation(self.default_shader, ident.as_ptr());
            let ident = std::ffi::CString::new("texture_sampler").unwrap();
            self.uniforms.texture_sampler = gl::GetUniformLocation(self.default_shader, ident.as_ptr());
            let ident = std::ffi::CString::new("transform_matrix").unwrap();
            self.uniforms.transform_matrix = gl::GetUniformLocation(self.default_shader, ident.as_ptr());
            let ident = std::ffi::CString::new("texture_transform_matrix").unwrap();
            self.uniforms.texture_transform_matrix = gl::GetUniformLocation(self.default_shader, ident.as_ptr());

            let draw_buffers: [u32; 1] = [gl::COLOR_ATTACHMENT0];
            gl::DrawBuffers(1, &draw_buffers[0]);
        }

        self.rect_vertex_buffer.add_vertex(&VertexImpl{x: 0.0, y: 0.0, color: color::Color{r: 255, g: 255, b: 255, a: 255}, tex_x: 0.0, tex_y: 0.0});
        self.rect_vertex_buffer.add_vertex(&VertexImpl{x: 1.0, y: 0.0, color: color::Color{r: 255, g: 255, b: 255, a: 255}, tex_x: 1.0, tex_y: 0.0});
        self.rect_vertex_buffer.add_vertex(&VertexImpl{x: 0.0, y: 1.0, color: color::Color{r: 255, g: 255, b: 255, a: 255}, tex_x: 0.0, tex_y: 1.0});

        self.rect_vertex_buffer.add_vertex(&VertexImpl{x: 1.0, y: 1.0, color: color::Color{r: 255, g: 255, b: 255, a: 255}, tex_x: 1.0, tex_y: 1.0});
        self.rect_vertex_buffer.add_vertex(&VertexImpl{x: 1.0, y: 0.0, color: color::Color{r: 255, g: 255, b: 255, a: 255}, tex_x: 1.0, tex_y: 0.0});
        self.rect_vertex_buffer.add_vertex(&VertexImpl{x: 0.0, y: 1.0, color: color::Color{r: 255, g: 255, b: 255, a: 255}, tex_x: 0.0, tex_y: 1.0});

        self.rect_vertex_buffer.upload();

        self.normalization_transform.stretch(2.0 / self.glfw_window.get_size().0 as f32, -2.0 / self.glfw_window.get_size().1 as f32);
        self.normalization_transform.translate(-self.glfw_window.get_size().0 as f32 / 2.0, -self.glfw_window.get_size().1 as f32 / 2.0);
    }

    /*
    Should be called before rendering.
    */
    pub fn pre_render(&self) {
        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
    }

    /*
    Should be called after rendering
    */
    pub fn post_render(&mut self) {
        self.glfw_window.swap_buffers();
        self.glfw.poll_events();
    }
}