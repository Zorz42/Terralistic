use crate::Rect;
use crate::shaders::compile_shader;
use crate::transformation::Transformation;

const BLUR_VERTEX_SHADER_CODE: &str = r#"
#version 330 core\n
layout(location = 0) in vec2 vertex_position;
out vec2 uv;
uniform mat3 transform_matrix;
uniform mat3 texture_transform_matrix;
void main() {
   gl_Position = vec4(transform_matrix * vec3(vertex_position.xy, 1), 1);
   uv = (texture_transform_matrix * vec3(vertex_position.xy, 1)).xy;
"#;

const BLUR_FRAGMENT_SHADER_CODE: &str = r#"
#version 330 core\n
in vec2 uv;
layout(location = 0) out vec4 color;
uniform sampler2D texture_sampler;
uniform vec2 blur_offset;
uniform vec4 limit;
const float gauss[17] = float[](0.0038, 0.0087, 0.0180, 0.0332, 0.0547, 0.0807, 0.1065, 0.1258, 0.1330, 0.1258, 0.1065, 0.0807, 0.0547, 0.0332, 0.0180, 0.0087, 0.0038);
void main() {
   color = vec4(0, 0, 0, 0);
   for(int i = 0; i < 17; i++)
       color += texture(texture_sampler, max(min(uv + (i - 8.0) * blur_offset, vec2(limit.x, limit.y)), vec2(limit.z, limit.w))) * gauss[i];
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
    pub(crate) blur_enabled: bool,
    pub(crate) anti_stutter: bool,
}

impl BlurContext {
    /**
    Creates a new blur context. Opengl context must be initialized.
     */
    pub(crate) fn new() -> Self {
        let blur_shader = compile_shader(BLUR_VERTEX_SHADER_CODE, BLUR_FRAGMENT_SHADER_CODE);
        BlurContext {
            blur_shader,
            transform_matrix_uniform: unsafe { gl::GetUniformLocation(blur_shader, "transform_matrix".as_ptr() as *const i8) },
            texture_transform_matrix_uniform: unsafe { gl::GetUniformLocation(blur_shader, "texture_transform_matrix".as_ptr() as *const i8) },
            texture_sampler_uniform: unsafe { gl::GetUniformLocation(blur_shader, "texture_sampler".as_ptr() as *const i8) },
            blur_offset_uniform: unsafe { gl::GetUniformLocation(blur_shader, "blur_offset".as_ptr() as *const i8) },
            limit_uniform: unsafe { gl::GetUniformLocation(blur_shader, "limit".as_ptr() as *const i8) },
            blur_enabled: true,
            anti_stutter: false,
        }
    }

    fn blur_rect(&self, offset_x: f32, offset_y: f32, texture: u32, back_texture: u32) {

        unsafe {
            //glActiveTexture(GL_TEXTURE0);
            gl::ActiveTexture(gl::TEXTURE0);
            //glBindTexture(GL_TEXTURE_2D, texture);
            gl::BindTexture(gl::TEXTURE_2D, texture);
            //glFramebufferTexture(GL_FRAMEBUFFER, GL_COLOR_ATTACHMENT0, back_texture, 0);
            gl::FramebufferTexture(gl::FRAMEBUFFER, gl::COLOR_ATTACHMENT0, back_texture, 0);
            //glUniform2f(gfx::uniform_blur_offset, 0, offset_y);
            gl::Uniform2f(self.blur_offset_uniform, 0.0, offset_y);
            //glDrawArrays(GL_TRIANGLES, 0, 6);
            gl::DrawArrays(gl::TRIANGLES, 0, 6);

            //glActiveTexture(GL_TEXTURE0);
            gl::ActiveTexture(gl::TEXTURE0);
            //glBindTexture(GL_TEXTURE_2D, back_texture);
            gl::BindTexture(gl::TEXTURE_2D, back_texture);
            //glFramebufferTexture(GL_FRAMEBUFFER, GL_COLOR_ATTACHMENT0, texture, 0);
            gl::FramebufferTexture(gl::FRAMEBUFFER, gl::COLOR_ATTACHMENT0, texture, 0);
            //glUniform2f(gfx::uniform_blur_offset, offset_x, 0);
            gl::Uniform2f(self.blur_offset_uniform, offset_x, 0.0);
            //glDrawArrays(GL_TRIANGLES, 0, 6);
            gl::DrawArrays(gl::TRIANGLES, 0, 6);
        }
    }

    /**
    Blurs region on a given texture.
     */
    pub(crate) fn blur_region(&self, rect: &Rect, radius: i32, gl_texture: u32, back_texture: u32, width: f32, height: f32, texture_transform: &Transformation) {
        //if(anti_stutter)
            //glFinish();
        if self.anti_stutter {
            unsafe {
                gl::Finish();
            }
        }

        //glEnableVertexAttribArray(SHADER_TEXTURE_COORD_BUFFER);
        unsafe {
            gl::EnableVertexAttribArray(2);
        }

        //glUseProgram(gfx::blur_shader_program);
        unsafe {
            gl::UseProgram(self.blur_shader);
        }

        //float x1 = (rect.x + 1) / width, y1 = (rect.y + 1) / height, x2 = (rect.x + rect.w - 1) / width, y2 = (rect.y + rect.h - 1) / height;
        let x1 = (rect.x as f32 + 1.0) / width;
        let y1 = (rect.y as f32 + 1.0) / height;
        let x2 = (rect.x as f32 + rect.w as f32 - 1.0) / width;
        let y2 = (rect.y as f32 + rect.h as f32 - 1.0) / height;
        //glUniform4f(gfx::uniform_blur_limit, x2, -y1, x1, -y2);
        unsafe {
            gl::Uniform4f(self.limit_uniform, x2, -y1, x1, -y2);
        }

        //glUniform1i(gfx::uniform_blur_texture_sampler, 0);
        unsafe {
            gl::Uniform1i(self.texture_sampler_uniform, 0);
        }

        //_Transformation transform = texture_transform;
        let mut transform = texture_transform.clone();
        //transform.translate(rect.x, rect.y);
        transform.translate(rect.x as f32, rect.y as f32);
        //transform.stretch(rect.w, rect.h);
        transform.stretch(rect.w as f32, rect.h as f32);
        //glUniformMatrix3fv(gfx::uniform_blur_transform_matrix, 1, GL_FALSE, transform.getArray());
        unsafe {
            gl::UniformMatrix3fv(self.transform_matrix_uniform, 1, gl::FALSE, transform.matrix.as_ptr());
        }

        //transform = gfx::_Transformation();
        transform = Transformation::new();
        //transform.stretch(1 / width, -1 / height);
        transform.stretch(1.0 / width, -1.0 / height);
        //transform.translate(rect.x, rect.y);
        transform.translate(rect.x as f32, rect.y as f32);
        //transform.stretch(rect.w, rect.h);
        transform.stretch(rect.w as f32, rect.h as f32);
        //glUniformMatrix3fv(gfx::uniform_blur_texture_transform_matrix, 1, GL_FALSE, transform.getArray());
        unsafe {
            gl::UniformMatrix3fv(self.texture_transform_matrix_uniform, 1, gl::FALSE, transform.matrix.as_ptr());
        }

        //glBindBuffer(GL_ARRAY_BUFFER, gfx::rect_vertex_buffer);
        unsafe {
            //gl::BindBuffer(gl::ARRAY_BUFFER, self.rect_vertex_buffer);
        }
        //glVertexAttribPointer(SHADER_VERTEX_BUFFER, 2, GL_FLOAT, GL_FALSE, 0, nullptr);
        unsafe {
            gl::VertexAttribPointer(0, 2, gl::FLOAT, gl::FALSE, 0, std::ptr::null());
        }
        //glVertexAttribPointer(SHADER_TEXTURE_COORD_BUFFER, 2, GL_FLOAT, GL_FALSE, 0, nullptr);
        unsafe {
            gl::VertexAttribPointer(2, 2, gl::FLOAT, gl::FALSE, 0, std::ptr::null());
        }


        //glUniform1i(gfx::uniform_texture_sampler, 0);
        unsafe {
            gl::Uniform1i(self.texture_sampler_uniform, 0);
        }
        //glUniform1i(gfx::uniform_has_texture, 1);
        unsafe {
            //gl::Uniform1i(self.has_texture_uniform, 1);
        }
        //glUniform1i(gfx::uniform_has_color_buffer, 0);
        unsafe {
            //gl::Uniform1i(self.has_color_buffer_uniform, 0);
        }

        //glUseProgram(gfx::blur_shader_program);
        unsafe {
            gl::UseProgram(self.blur_shader);
        }

        //while(radius > 10) {
            //blurRect(radius / width / 10, radius / height / 10, texture, back_texture);
            //radius = std::sqrt(radius) * BLUR_QUALITY;
        //}
        let mut radius = radius;
        while radius > 10 {
            self.blur_rect((radius as f32 / width / 10.0) as f32, (radius as f32 / height / 10.0) as f32, gl_texture, back_texture);
            //radius = (radius as f32 * BLUR_QUALITY).sqrt() as i32;
        }

        //glUseProgram(gfx::shader_program);
        unsafe {
            //gl::UseProgram(self.shader);
        }

        //glDisableVertexAttribArray(SHADER_TEXTURE_COORD_BUFFER);
        unsafe {
            gl::DisableVertexAttribArray(2);
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