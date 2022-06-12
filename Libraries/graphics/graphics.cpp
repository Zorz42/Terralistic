#include "glfwAbstraction.hpp"
#include "graphics-internal.hpp"
#include <thread>
#include <cmath>

const char* blur_vertex_shader_code =
"#version 330 core\n"
"layout(location = 0) in vec2 vertex_position;"
"out vec2 uv;"
"uniform mat3 transform_matrix;"
"uniform mat3 texture_transform_matrix;"
"void main() {"
"   gl_Position = vec4(transform_matrix * vec3(vertex_position.xy, 1), 1);"
"   uv = (texture_transform_matrix * vec3(vertex_position.xy, 1)).xy;"
"}";

const char* blur_fragment_shader_code =
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

void gfx::setMinimumWindowSize(int width, int height) {
    window_width_min = width;
    window_height_min = height;
    float scale = 1;
#ifndef __APPLE__
    scale = gfx::global_scale;
#endif
    glfwSetWindowSizeLimits(glfw_window, width * scale, height * scale, -1, -1);
}

void gfx::init(int window_width_, int window_height_) {
    initGlfw(window_width_, window_height_);
    
    glfwSetKeyCallback(glfw_window, gfx::keyCallback);
    glfwSetScrollCallback(glfw_window, gfx::scrollCallback);
    glfwSetCharCallback(glfw_window, gfx::characterCallback);
    glfwSetMouseButtonCallback(glfw_window, gfx::mouseButtonCallback);

    blur_shader_program = CompileShaders(blur_vertex_shader_code, blur_fragment_shader_code);
    uniform_blur_transform_matrix = glGetUniformLocation(blur_shader_program, "transform_matrix");
    uniform_blur_texture_transform_matrix = glGetUniformLocation(blur_shader_program, "texture_transform_matrix");
    uniform_blur_texture_sampler = glGetUniformLocation(blur_shader_program, "texture_sampler");
    uniform_blur_offset = glGetUniformLocation(blur_shader_program, "blur_offset");
    uniform_blur_limit = glGetUniformLocation(blur_shader_program, "limit");
    
    glEnableVertexAttribArray(SHADER_VERTEX_BUFFER);

    shadow_texture = new Texture;
    shadow_texture->createBlankImage(700, 700);
    Texture shadow_texture_back;
    shadow_texture_back.createBlankImage(700, 700);
    shadow_texture->setRenderTarget();
    RectShape(200, 200, 300, 300).render({0, 0, 0});
    
    _Transformation shadow_transform = shadow_texture->getNormalizationTransform();
    shadow_transform.translate(-700, 0);
    shadow_transform.stretch(2, 2);
    for(int i = 0; i < 10; i++)
        blurRectangle(RectShape(0, 0, 700, 700), GFX_SHADOW_BLUR, shadow_texture->getGlTexture(), shadow_texture_back.getGlTexture(), 700, 700, shadow_transform);
}

void gfx::enableVsync(bool enabled) {
    if(enabled)
        glfwSwapInterval(1);
    else
        glfwSwapInterval(0);
}

bool fontColEmpty(const unsigned char* data, int x, int y) {
    int curr_index = (256 - y) * 256 + x;
    for(int i = 0; i < 16; i++) {
        if(data[curr_index * 4 + 3] != 0)
            return false;
        curr_index -= 256;
    }
    
    return true;
}

void gfx::loadFont(const unsigned char* data) {
    font_texture = new Texture;
    font_texture->loadFromData(data, 256, 256);
    for(int y = 0; y < 16; y++)
        for(int x = 0; x < 16; x++) {
            RectShape rect(x * 16, y * 16, 16, 16);
            
            while(rect.w > 0 && fontColEmpty(data, rect.x, rect.y)) {
                rect.x++;
                rect.w--;
            }
            
            while(rect.w > 0 && fontColEmpty(data, rect.x + rect.w - 1, rect.y))
                rect.w--;
            
            if(y * 16 + x == ' ') {
                rect.x = x * 16;
                rect.w = 2;
            }
            
            font_rects[y * 16 + x] = rect;
        }
}

void gfx::quit() {
    delete font_texture;
    delete shadow_texture;
    glfwTerminate();
}

void gfx::addAGlobalUpdateFunction(GlobalUpdateFunction* global_update_function) {
    global_update_functions.push_back(global_update_function);
}

int gfx::getWindowWidth() {
    return window_width;
}

int gfx::getWindowHeight() {
    return window_height;
}

void gfx::updateWindow() {
    glBindFramebuffer(GL_FRAMEBUFFER, 0);
    glViewport(0, 0, window_width * global_scale_x, window_height * global_scale_y);
    
    _Transformation texture_transform = window_normalization_transform;
    texture_transform.stretch(window_width * 0.5f, window_height * 0.5f);
    glUniformMatrix3fv(uniform_texture_transform_matrix, 1, GL_FALSE, texture_transform.getArray());
    
    glActiveTexture(GL_TEXTURE0);
    glBindTexture(GL_TEXTURE_2D, window_texture);
    
    glUniform1i(uniform_texture_sampler, 0);
    glUniform1i(uniform_has_texture, 1);
    glUniform1i(uniform_has_color_buffer, 0);
    glUniform1i(uniform_blend_multiply, 0);
    _Transformation transform = normalization_transform;
    
    transform.stretch(window_width, window_height);
    
    glUniformMatrix3fv(uniform_transform_matrix, 1, GL_FALSE, transform.getArray());
    glUniform4f(uniform_default_color, 1.f, 1.f, 1.f, 1.f);

    glEnableVertexAttribArray(SHADER_TEXTURE_COORD_BUFFER);
    
    glBindBuffer(GL_ARRAY_BUFFER, rect_vertex_buffer);
    glVertexAttribPointer(SHADER_VERTEX_BUFFER, 2, GL_FLOAT, GL_FALSE, 0, nullptr);
    
    glBindBuffer(GL_ARRAY_BUFFER, rect_vertex_buffer);
    glVertexAttribPointer(SHADER_TEXTURE_COORD_BUFFER, 2, GL_FLOAT, GL_FALSE, 0, nullptr);

    glDrawArrays(GL_TRIANGLES, 0, 6);
    
    glDisableVertexAttribArray(SHADER_TEXTURE_COORD_BUFFER);
    
    glfwSwapBuffers(glfw_window);
    
    glBindFramebuffer(GL_FRAMEBUFFER, default_framebuffer);
}

void gfx::sleep(float ms) {
    std::this_thread::sleep_for(std::chrono::microseconds(int(ms * 1000)));
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
