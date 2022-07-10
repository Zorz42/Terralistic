#include <cmath>
#include "glfwAbstraction.hpp"
#include "blur.hpp"

static const char* blur_vertex_shader_code =
"#version 330 core\n"
"layout(location = 0) in vec2 vertex_position;"
"out vec2 uv;"
"uniform mat3 transform_matrix;"
"uniform mat3 texture_transform_matrix;"
"void main() {"
"   gl_Position = vec4(transform_matrix * vec3(vertex_position.xy, 1), 1);"
"   uv = (texture_transform_matrix * vec3(vertex_position.xy, 1)).xy;"
"}";

static const char* blur_fragment_shader_code =
"#version 330 core\n"
"in vec2 uv;"
"layout(location = 0) out vec4 color;"
"uniform sampler2D texture_sampler;"
"uniform vec2 blur_offset;"
"uniform vec4 limit;"
"uniform mat3 transform_matrix;"
"uniform mat3 texture_transform_matrix;"
"float gauss[21] = float[](0.0012, 0.0015, 0.0038, 0.0087, 0.0180, 0.0332, 0.0547, 0.0807, 0.1065, 0.1258, 0.1330, 0.1258, 0.1065, 0.0807, 0.0547, 0.0332, 0.0180, 0.0087, 0.0038, 0.0015, 0.0012);"
"void main() {"
"   color = vec4(0, 0, 0, 0);"
"   for(int i = 0; i < 21; i++)"
"       color += texture(texture_sampler, max(min(uv + (i - 10.0) * blur_offset, vec2(limit.x, limit.y)), vec2(limit.z, limit.w))) * gauss[i];"
"}";

void gfx::initBlur() {
    blur_shader_program = CompileShaders(blur_vertex_shader_code, blur_fragment_shader_code);
    uniform_blur_transform_matrix = glGetUniformLocation(blur_shader_program, "transform_matrix");
    uniform_blur_texture_transform_matrix = glGetUniformLocation(blur_shader_program, "texture_transform_matrix");
    uniform_blur_texture_sampler = glGetUniformLocation(blur_shader_program, "texture_sampler");
    uniform_blur_offset = glGetUniformLocation(blur_shader_program, "blur_offset");
    uniform_blur_limit = glGetUniformLocation(blur_shader_program, "limit");
}

void blurRect(float offset_x, float offset_y, GLuint texture, GLuint back_texture) {
    glActiveTexture(GL_TEXTURE0);
    glBindTexture(GL_TEXTURE_2D, texture);
    glFramebufferTexture(GL_FRAMEBUFFER, GL_COLOR_ATTACHMENT0, back_texture, 0);
    glUniform2f(gfx::uniform_blur_offset, 0, offset_y);
    glDrawArrays(GL_TRIANGLES, 0, 6);
    
    glActiveTexture(GL_TEXTURE0);
    glBindTexture(GL_TEXTURE_2D, back_texture);
    glFramebufferTexture(GL_FRAMEBUFFER, GL_COLOR_ATTACHMENT0, texture, 0);
    glUniform2f(gfx::uniform_blur_offset, offset_x, 0);
    glDrawArrays(GL_TRIANGLES, 0, 6);
}

#define BLUR_QUALITY 3

void gfx::blurRectangle(RectShape rect, int radius, unsigned int texture, unsigned int back_texture, float width, float height, _Transformation texture_transform) {
    updated_back_window_texture = false;
    glEnableVertexAttribArray(SHADER_TEXTURE_COORD_BUFFER);
    
    glUseProgram(gfx::blur_shader_program);
    
    float x1 = (rect.x + 1) / width, y1 = (rect.y + 1) / height, x2 = (rect.x + rect.w - 1) / width, y2 = (rect.y + rect.h - 1) / height;
    glUniform4f(gfx::uniform_blur_limit, x2, -y1, x1, -y2);
    
    glUniform1i(gfx::uniform_blur_texture_sampler, 0);
    glUniform1i(gfx::uniform_back_texture_sampler, 0);
    
    _Transformation transform = texture_transform;
    transform.translate(rect.x, rect.y);
    transform.stretch(rect.w, rect.h);
    glUniformMatrix3fv(gfx::uniform_blur_transform_matrix, 1, GL_FALSE, transform.getArray());
    
    transform = gfx::_Transformation();
    transform.stretch(1 / width, -1 / height);
    transform.translate(rect.x, rect.y);
    transform.stretch(rect.w, rect.h);
    glUniformMatrix3fv(gfx::uniform_blur_texture_transform_matrix, 1, GL_FALSE, transform.getArray());
    
    glBindBuffer(GL_ARRAY_BUFFER, gfx::rect_vertex_buffer);
    glVertexAttribPointer(SHADER_VERTEX_BUFFER, 2, GL_FLOAT, GL_FALSE, 0, nullptr);
    glVertexAttribPointer(SHADER_TEXTURE_COORD_BUFFER, 2, GL_FLOAT, GL_FALSE, 0, nullptr);
    
    glUniform1i(gfx::uniform_texture_sampler, 0);
    glUniform1i(gfx::uniform_has_texture, 1);
    glUniform1i(gfx::uniform_has_color_buffer, 0);
    
    glUseProgram(gfx::blur_shader_program);
    
    while(radius > 10) {
        blurRect(radius / width / 10, radius / height / 10, texture, back_texture);
        radius = std::sqrt(radius) * BLUR_QUALITY;
    }
    
    glUseProgram(gfx::shader_program);
    
    glDisableVertexAttribArray(SHADER_TEXTURE_COORD_BUFFER);
}
