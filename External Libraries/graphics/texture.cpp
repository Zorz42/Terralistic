#include "graphics-internal.hpp"

void gfx::Texture::createBlankImage(int width_, int height_) {
    if(width_ <= 0 || height_ <= 0)
        throw std::runtime_error("Width and Height of a texture size must be positive.");
    
    unsigned char* array = new unsigned char[width_ * height_ * 4];
    memset(array, 0, width_ * height_ * 4);
    loadFromData(array, width_, height_);
    delete[] array;
}

void gfx::Texture::loadFromData(const unsigned char* data, int width_, int height_) {
    if(width_ <= 0 || height_ <= 0)
        throw std::runtime_error("Width and Height of a texture size must be positive.");
    
    freeTexture();
    
    width = width_;
    height = height_;
    
    glGenTextures(1, &gl_texture);
    glBindTexture(GL_TEXTURE_2D, gl_texture);
    glTexImage2D(GL_TEXTURE_2D, 0, GL_RGBA, width, height, 0, GL_BGRA, GL_UNSIGNED_BYTE, data);
    
    glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, GL_NEAREST);
    glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, GL_NEAREST);
    
    texture_normalization_transform = gfx::Transformation();
    texture_normalization_transform.translate(0.f, 1.f);
    texture_normalization_transform.stretch(1.f / width, -1.f / height);
}

gfx::Texture::~Texture() {
    freeTexture();
}

void gfx::Texture::freeTexture() {
    if(gl_texture != -1) {
        glDeleteTextures(1, &gl_texture);
        gl_texture = -1;
        width = 0;
        height = 0;
    }
}

int gfx::Texture::getTextureWidth() const {
    return width;
}

int gfx::Texture::getTextureHeight() const {
    return height;
}

void gfx::Texture::render(float scale, int x, int y, RectShape src_rect, bool flipped, Color color) const {
    if(scale <= 0)
        throw std::runtime_error("Texture scale must be positive.");
    
    Transformation texture_transform = texture_normalization_transform;
    texture_transform.translate(src_rect.x, src_rect.y);
    texture_transform.stretch(src_rect.w, src_rect.h);
    glUniformMatrix3fv(uniform_texture_transform_matrix, 1, GL_FALSE, texture_transform.getArray());
    
    glActiveTexture(GL_TEXTURE0);
    glBindTexture(GL_TEXTURE_2D, gl_texture);
    
    glUniform1i(uniform_texture_sampler, 0);
    glUniform1i(uniform_has_texture, 1);
    glUniform1i(uniform_has_color_buffer, 0);
    Transformation transform = normalization_transform;
    
    if(flipped) {
        transform.translate(src_rect.w * scale + x * 2, 0);
        transform.stretch(-1, 1);
    }
    transform.translate(x, y);
    transform.stretch(src_rect.w * scale, src_rect.h * scale);
    
    glUniformMatrix3fv(uniform_transform_matrix, 1, GL_FALSE, transform.getArray());
    glUniform4f(uniform_default_color, color.r * (1 / 256.f), color.g * (1 / 256.f), color.b * (1 / 256.f), color.a * (1 / 256.f));

    glEnableVertexAttribArray(2);
    
    glBindBuffer(GL_ARRAY_BUFFER, rect_vertex_buffer);
    glVertexAttribPointer(SHADER_VERTEX_BUFFER, 2, GL_FLOAT, GL_FALSE, 0, nullptr);
    
    glBindBuffer(GL_ARRAY_BUFFER, rect_vertex_buffer);
    glVertexAttribPointer(SHADER_TEXTURE_COORD_BUFFER, 2, GL_FLOAT, GL_FALSE, 0, nullptr);

    glDrawArrays(GL_TRIANGLES, 0, 6);
    
    glDisableVertexAttribArray(2);
}

void gfx::Texture::render(float scale, int x, int y, bool flipped, Color color) const {
    render(scale, x, y, {0, 0, getTextureWidth(), getTextureHeight()}, flipped, color);
}

GLuint gfx::Texture::getGlTexture() {
    return gl_texture;
}

void gfx::Texture::setRenderTarget() {
    glFramebufferTexture(GL_FRAMEBUFFER, GL_COLOR_ATTACHMENT0, gl_texture, 0);
    glViewport(0, 0, width, height);
    normalization_transform = texture_normalization_transform;
    normalization_transform.translate(-width, 0);
    normalization_transform.stretch(2, 2);
}

void gfx::resetRenderTarget() {
    glFramebufferTexture(GL_FRAMEBUFFER, GL_COLOR_ATTACHMENT0, window_texture, 0);
    glViewport(0, 0, window_width, window_height);
    normalization_transform = window_normalization_transform;
}

#define TEXT_SPACING 1

void gfx::Texture::loadFromText(const std::string& text, Color color) {
    int width_ = 0;
    for(int i = 0; i < text.size(); i++)
        width_ += font_rects[(int)text[i]].w + TEXT_SPACING;
    
    createBlankImage(width_, 16);
    setRenderTarget();
    int x = 0;
    for(int i = 0; i < text.size(); i++) {
        font_texture.render(1, x, 0, font_rects[(int)text[i]], false, color);
        x += font_rects[(int)text[i]].w + TEXT_SPACING;
    }
    resetRenderTarget();
}

const gfx::Transformation& gfx::Texture::getNormalizationTransform() const {
    return texture_normalization_transform;
}

GLuint gfx::Texture::getGlTexture() const {
    return gl_texture;
}
