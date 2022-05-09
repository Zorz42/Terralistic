#include "graphics-internal.hpp"

gfx::RectShape::RectShape(int x, int y, int w, int h) : x(x), y(y), w(w), h(h) {
    if(w < 0 || h < 0)
        throw std::runtime_error("RectShape width and height must be positive.");
}

void gfx::RectShape::render(Color color) const {
    if(color.a == 0)
        return;
    
    glBindBuffer(GL_ARRAY_BUFFER, rect_vertex_buffer);
    glVertexAttribPointer(SHADER_VERTEX_BUFFER, 2, GL_FLOAT, GL_FALSE, 0, nullptr);
    
    glUniform1i(uniform_has_texture, 0);
    glUniform1i(uniform_blend_multiply, 0);
    glUniform1i(uniform_has_color_buffer, 0);
    Transformation transform = normalization_transform;
    transform.translate(x, y);
    transform.stretch(w, h);
    
    glUniformMatrix3fv(uniform_transform_matrix, 1, GL_FALSE, transform.getArray());
    
    glUniform4f(uniform_default_color, color.r * (1 / 256.f), color.g * (1 / 256.f), color.b * (1 / 256.f), color.a * (1 / 256.f));
    
    glDrawArrays(GL_TRIANGLES, 0, 6);
}

void gfx::RectShape::renderOutline(Color color) const {
    if(color.a == 0)
        return;
    
    glBindBuffer(GL_ARRAY_BUFFER, rect_outline_vertex_buffer);
    glVertexAttribPointer(SHADER_VERTEX_BUFFER, 2, GL_FLOAT, GL_FALSE, 0, nullptr);
    
    glUniform1i(uniform_has_texture, 0);
    glUniform1i(uniform_blend_multiply, 0);
    glUniform1i(uniform_has_color_buffer, 0);
    Transformation transform = normalization_transform;
    transform.translate(x, y);
    transform.stretch(w, h);
    
    glUniformMatrix3fv(uniform_transform_matrix, 1, GL_FALSE, transform.getArray());
    
    glUniform4f(uniform_default_color, color.r * (1 / 256.f), color.g * (1 / 256.f), color.b * (1 / 256.f), color.a * (1 / 256.f));
    
    glDrawArrays(GL_LINE_LOOP, 0, 4);
}
