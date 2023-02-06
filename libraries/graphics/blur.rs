use super::shaders::compile_shader;
use super::transformation::Transformation;
use super::Rect;
use std::ffi::c_void;

const BLUR_VERTEX_SHADER_CODE: &str = r#"
#version 330 core

layout(location = 0) in vec2 vertex_position;
out vec2 uv;
uniform mat3 transform_matrix;
uniform mat3 texture_transform_matrix;

void main() {
   gl_Position = vec4(transform_matrix * vec3(vertex_position.xy, 1), 1);
   uv = (texture_transform_matrix * vec3(vertex_position.xy, 1)).xy;
}
"#;

const BLUR_FRAGMENT_SHADER_CODE: &str = r#"
#version 330 core

in vec2 uv;
layout(location = 0) out vec4 color;
uniform sampler2D texture_sampler;
uniform vec2 blur_offset;
uniform vec4 limit;
const float gauss[17] = float[](0.0038, 0.0087, 0.0180, 0.0332, 0.0547, 0.0807, 0.1065, 0.1258, 0.1330, 0.1258, 0.1065, 0.0807, 0.0547, 0.0332, 0.0180, 0.0087, 0.0038);

void main() {
   color = vec4(0, 0, 0, 255);
   for(int i = 0; i < 17; i++)
       color += texture(texture_sampler, max(min(uv + (i - 8.0) * blur_offset, vec2(limit.x, limit.y)), vec2(limit.z, limit.w))) * gauss[i];
}
"#;

/**
Blur context struct holds shaders needed for blurring and
all uniform handles.
 */
pub(crate) struct BlurContext {
    blur_shader: u32,
    transform_matrix_uniform: i32,
    texture_transform_matrix_uniform: i32,
    texture_sampler_uniform: i32,
    blur_offset_uniform: i32,
    limit_uniform: i32,
    rect_vertex_buffer: u32,
    pub(crate) blur_enabled: bool,
    pub(crate) anti_stutter: bool,
}

const BLUR_QUALITY: f32 = 3.0;

impl BlurContext {
    /**
    Creates a new blur context. Opengl context must be initialized.
     */
    pub(crate) fn new() -> Self {
        let blur_shader = compile_shader(BLUR_VERTEX_SHADER_CODE, BLUR_FRAGMENT_SHADER_CODE);
        BlurContext {
            blur_shader,
            transform_matrix_uniform: unsafe {
                gl::GetUniformLocation(blur_shader, "transform_matrix\0".as_ptr() as *const i8)
            },
            texture_transform_matrix_uniform: unsafe {
                gl::GetUniformLocation(
                    blur_shader,
                    "texture_transform_matrix\0".as_ptr() as *const i8,
                )
            },
            texture_sampler_uniform: unsafe {
                gl::GetUniformLocation(blur_shader, "texture_sampler\0".as_ptr() as *const i8)
            },
            blur_offset_uniform: unsafe {
                gl::GetUniformLocation(blur_shader, "blur_offset\0".as_ptr() as *const i8)
            },
            limit_uniform: unsafe {
                gl::GetUniformLocation(blur_shader, "limit\0".as_ptr() as *const i8)
            },
            rect_vertex_buffer: {
                let mut buffer = 0;
                unsafe {
                    gl::GenBuffers(1, &mut buffer);

                    let rect_vertex_array: [f32; 12] =
                        [0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 1.0, 0.0, 0.0, 1.0, 1.0, 1.0];

                    gl::BindBuffer(gl::ARRAY_BUFFER, buffer);
                    gl::BufferData(
                        gl::ARRAY_BUFFER,
                        4 * 12,
                        rect_vertex_array.as_ptr() as *const c_void,
                        gl::STATIC_DRAW,
                    );
                    gl::BindBuffer(gl::ARRAY_BUFFER, 0);
                }
                buffer
            },
            blur_enabled: true,
            anti_stutter: false,
        }
    }

    /**
    Only applies blur shader pass to the given texture.
     */
    fn blur_rect(&self, offset_x: f32, offset_y: f32, gl_texture1: u32, gl_texture2: u32) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, gl_texture1);
            gl::FramebufferTexture(gl::FRAMEBUFFER, gl::COLOR_ATTACHMENT0, gl_texture2, 0);
            gl::Uniform2f(self.blur_offset_uniform, offset_x, offset_y);
            gl::DrawArrays(gl::TRIANGLES, 0, 6);
        }
    }

    /**
    Blurs region on a given texture.
     */
    pub(crate) fn blur_region(
        &self,
        mut rect: Rect,
        radius: i32,
        gl_texture: u32,
        back_texture: u32,
        size: (f32, f32),
        texture_transform: &Transformation,
    ) {
        if !self.blur_enabled {
            return;
        }

        unsafe {
            if self.anti_stutter {
                gl::Finish();
            }

            let mut x1 = rect.x;
            let mut y1 = rect.y;
            let mut x2 = rect.x + rect.w;
            let mut y2 = rect.y + rect.h;

            x1 = x1.max(0);
            y1 = y1.max(0);
            x2 = x2.min(size.0 as i32);
            y2 = y2.min(size.1 as i32);

            rect.x = x1;
            rect.y = y1;
            rect.w = x2 - x1;
            rect.h = y2 - y1;

            if rect.w <= 0 || rect.h <= 0 {
                return;
            }

            gl::UseProgram(self.blur_shader);

            gl::EnableVertexAttribArray(0);

            let x1 = (rect.x as f32 + 1.0) / size.0;
            let y1 = (rect.y as f32 + 1.0) / size.1;
            let x2 = (rect.x as f32 + rect.w as f32 - 1.0) / size.0;
            let y2 = (rect.y as f32 + rect.h as f32 - 1.0) / size.1;

            gl::Uniform4f(self.limit_uniform, x2, -y1, x1, -y2);
            gl::Uniform1i(self.texture_sampler_uniform, 0);

            let mut transform = texture_transform.clone();
            transform.translate(rect.x as f32, rect.y as f32);
            transform.stretch(rect.w as f32, rect.h as f32);

            gl::UniformMatrix3fv(
                self.transform_matrix_uniform,
                1,
                gl::FALSE,
                transform.matrix.as_ptr(),
            );

            transform = Transformation::new();
            transform.stretch(1.0 / size.0, -1.0 / size.1);
            transform.translate(rect.x as f32, rect.y as f32);
            transform.stretch(rect.w as f32, rect.h as f32);

            gl::UniformMatrix3fv(
                self.texture_transform_matrix_uniform,
                1,
                gl::FALSE,
                transform.matrix.as_ptr(),
            );
            gl::BindBuffer(gl::ARRAY_BUFFER, self.rect_vertex_buffer);
            gl::VertexAttribPointer(0, 2, gl::FLOAT, gl::FALSE, 0, std::ptr::null());

            let mut radius = radius;
            while radius > 10 {
                self.blur_rect(0.0, radius as f32 / size.1 / 10.0, gl_texture, back_texture);
                self.blur_rect(radius as f32 / size.0 / 10.0, 0.0, back_texture, gl_texture);
                radius = (radius as f32 * BLUR_QUALITY).sqrt() as i32;
            }

            gl::DisableVertexAttribArray(0);
        }
    }
}

/**
Drop function for blur context.
 */
impl Drop for BlurContext {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.blur_shader);
        }
    }
}
