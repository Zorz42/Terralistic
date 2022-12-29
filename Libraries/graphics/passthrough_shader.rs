use crate::Color;
use crate::shaders::compile_shader;
use crate::vertex_buffer::{VertexBuffer, Vertex};

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
Passthrough shader context struct holds shaders needed for drawing
rectangles and all uniform handles. It also holds a vertex buffer
for drawing rectangles.
 */
pub(crate) struct PassthroughShader {
    pub passthrough_shader: u32,
    pub rect_vertex_buffer: VertexBuffer,
    pub rect_outline_vertex_buffer: VertexBuffer,
    pub has_texture: i32,
    pub global_color: i32,
    pub transform_matrix: i32,
    pub texture_transform_matrix: i32,
}

impl PassthroughShader {
    /**
    Creates a new passthrough shader context. Opengl context must be initialized.
     */
    pub(crate) fn new() -> Self {
        let passthrough_shader = compile_shader(VERTEX_SHADER_CODE, FRAGMENT_SHADER_CODE);
        let mut rect_vertex_buffer = VertexBuffer::new();
        let mut rect_outline_vertex_buffer = VertexBuffer::new();

        rect_vertex_buffer.add_vertex(&Vertex { x: 0.0, y: 0.0, color: Color { r: 255, g: 255, b: 255, a: 255 }, tex_x: 0.0, tex_y: 0.0 });
        rect_vertex_buffer.add_vertex(&Vertex { x: 1.0, y: 0.0, color: Color { r: 255, g: 255, b: 255, a: 255 }, tex_x: 1.0, tex_y: 0.0 });
        rect_vertex_buffer.add_vertex(&Vertex { x: 0.0, y: 1.0, color: Color { r: 255, g: 255, b: 255, a: 255 }, tex_x: 0.0, tex_y: 1.0 });

        rect_vertex_buffer.add_vertex(&Vertex { x: 1.0, y: 1.0, color: Color { r: 255, g: 255, b: 255, a: 255 }, tex_x: 1.0, tex_y: 1.0 });
        rect_vertex_buffer.add_vertex(&Vertex { x: 1.0, y: 0.0, color: Color { r: 255, g: 255, b: 255, a: 255 }, tex_x: 1.0, tex_y: 0.0 });
        rect_vertex_buffer.add_vertex(&Vertex { x: 0.0, y: 1.0, color: Color { r: 255, g: 255, b: 255, a: 255 }, tex_x: 0.0, tex_y: 1.0 });

        rect_vertex_buffer.upload();

        rect_outline_vertex_buffer.add_vertex(&Vertex { x: 0.0, y: 0.0, color: Color { r: 255, g: 255, b: 255, a: 255 }, tex_x: 0.0, tex_y: 0.0 });
        rect_outline_vertex_buffer.add_vertex(&Vertex { x: 1.0, y: 0.0, color: Color { r: 255, g: 255, b: 255, a: 255 }, tex_x: 1.0, tex_y: 0.0 });

        rect_outline_vertex_buffer.add_vertex(&Vertex { x: 1.0, y: 0.0, color: Color { r: 255, g: 255, b: 255, a: 255 }, tex_x: 1.0, tex_y: 0.0 });
        rect_outline_vertex_buffer.add_vertex(&Vertex { x: 1.0, y: 1.0, color: Color { r: 255, g: 255, b: 255, a: 255 }, tex_x: 1.0, tex_y: 1.0 });

        rect_outline_vertex_buffer.add_vertex(&Vertex { x: 1.0, y: 1.0, color: Color { r: 255, g: 255, b: 255, a: 255 }, tex_x: 1.0, tex_y: 1.0 });
        rect_outline_vertex_buffer.add_vertex(&Vertex { x: 0.0, y: 1.0, color: Color { r: 255, g: 255, b: 255, a: 255 }, tex_x: 0.0, tex_y: 1.0 });

        rect_outline_vertex_buffer.add_vertex(&Vertex { x: 0.0, y: 1.0, color: Color { r: 255, g: 255, b: 255, a: 255 }, tex_x: 0.0, tex_y: 1.0 });
        rect_outline_vertex_buffer.add_vertex(&Vertex { x: 0.0, y: 0.0, color: Color { r: 255, g: 255, b: 255, a: 255 }, tex_x: 0.0, tex_y: 0.0 });

        rect_outline_vertex_buffer.upload();

        let has_texture = unsafe { gl::GetUniformLocation(passthrough_shader, "has_texture\0".as_ptr() as *const i8) };
        let global_color = unsafe { gl::GetUniformLocation(passthrough_shader, "global_color\0".as_ptr() as *const i8) };
        let transform_matrix = unsafe { gl::GetUniformLocation(passthrough_shader, "transform_matrix\0".as_ptr() as *const i8) };
        let texture_transform_matrix = unsafe { gl::GetUniformLocation(passthrough_shader, "texture_transform_matrix\0".as_ptr() as *const i8) };

        PassthroughShader {
            passthrough_shader,
            rect_vertex_buffer,
            rect_outline_vertex_buffer,
            has_texture,
            global_color,
            transform_matrix,
            texture_transform_matrix,
        }
    }
}
