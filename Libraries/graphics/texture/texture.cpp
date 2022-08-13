#include <cstring>
#include "glfwAbstraction.hpp"
#include "exception.hpp"
#include "texture.hpp"
#include "font.hpp"

void gfx::Texture::loadFromSurface(const Surface& surface) {
    freeTexture();
    
    texture_width = surface.getWidth();
    texture_height = surface.getHeight();
    
    std::vector<unsigned char> data2(texture_width * texture_height * 4);
    for(int i = 0; i < texture_width * texture_height * 4; i++) {
        data2[i] = 255;
        if(i % 8 == 1)
            data2[i] = 0;
    }

    glGenTextures(1, &gl_texture);
    glBindTexture(GL_TEXTURE_2D, gl_texture);
    glTexImage2D(GL_TEXTURE_2D, 0, GL_RGBA, texture_width, texture_height, 0, GL_BGRA, GL_UNSIGNED_BYTE, &surface.getData()[0]);

    glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, GL_NEAREST);
    glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, GL_NEAREST);
}

gfx::Texture::~Texture() {
    freeTexture();
}

void gfx::Texture::freeTexture() {
    if(gl_texture != -1) {
        glDeleteTextures(1, &gl_texture);
        gl_texture = -1;
        texture_width = 0;
        texture_height = 0;
    }
}

int gfx::Texture::getTextureWidth() const {
    return texture_width;
}

int gfx::Texture::getTextureHeight() const {
    return texture_height;
}

void gfx::Texture::render(float scale, int x, int y, RectShape src_rect, bool flipped, Color color) const {
    if(scale <= 0)
        throw std::runtime_error("Texture scale must be positive.");
    
    _Transformation texture_transform = texture_normalization_transform;
    texture_transform.translate(src_rect.x, src_rect.y);
    texture_transform.stretch(src_rect.w, src_rect.h);
    glUniformMatrix3fv(uniform_texture_transform_matrix, 1, GL_FALSE, texture_transform.getArray());
    
    glActiveTexture(GL_TEXTURE0);
    glBindTexture(GL_TEXTURE_2D, gl_texture);
    
    glUniform1i(uniform_texture_sampler, 0);
    glUniform1i(uniform_has_texture, 1);
    glUniform1i(uniform_has_color_buffer, 0);
    _Transformation transform = normalization_transform;
    
    if(flipped) {
        transform.translate(src_rect.w * scale + x * 2, 0);
        transform.stretch(-1, 1);
    }
    
    transform.translate(x, y);
    transform.stretch(src_rect.w * scale, src_rect.h * scale);
    
    glUniformMatrix3fv(uniform_transform_matrix, 1, GL_FALSE, transform.getArray());
    glUniform4f(uniform_default_color, color.r * (1 / 256.f), color.g * (1 / 256.f), color.b * (1 / 256.f), color.a * (1 / 256.f));

    glEnableVertexAttribArray(SHADER_TEXTURE_COORD_BUFFER);
    
    glBindBuffer(GL_ARRAY_BUFFER, rect_vertex_buffer);
    glVertexAttribPointer(SHADER_VERTEX_BUFFER, 2, GL_FLOAT, GL_FALSE, 0, nullptr);
    
    glBindBuffer(GL_ARRAY_BUFFER, rect_vertex_buffer);
    glVertexAttribPointer(SHADER_TEXTURE_COORD_BUFFER, 2, GL_FLOAT, GL_FALSE, 0, nullptr);

    glDrawArrays(GL_TRIANGLES, 0, 6);
    
    glDisableVertexAttribArray(SHADER_TEXTURE_COORD_BUFFER);
}

void gfx::Texture::render(float scale, int x, int y, bool flipped, Color color) const {
    render(scale, x, y, {0, 0, getTextureWidth(), getTextureHeight()}, flipped, color);
}

unsigned int gfx::Texture::_getGlTexture() const {
    return gl_texture;
}

const gfx::_Transformation& gfx::Texture::_getNormalizationTransform() const {
    return texture_normalization_transform;
}
