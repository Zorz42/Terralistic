extern crate glfw;

const VERTEX_SHADER_CODE: &str =
"
#version 330 core\n
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
";

const FRAGMENT_SHADER_CODE: &str =
"
#version 330 core\n
in vec4 fragment_color;
in vec2 uv;
layout(location = 0) out vec4 color;
uniform sampler2D texture_sampler;
uniform int has_texture;
void main() {
   color = mix(vec4(1.f, 1.f, 1.f, 1.f), texture(texture_sampler, uv).rgba, has_texture) * fragment_color;
}
";

/*
This stores all values needed for OpenGL and GLFW.
*/
pub struct GlfwContext {

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

    let (mut window, events) = glfw.create_window(window_width as u32, window_height as u32, window_title.as_str(), glfw::WindowMode::Windowed).expect("Failed to create GLFW window.");

    while !window.should_close() {
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            
        }
    }

    GlfwContext{

    }
}