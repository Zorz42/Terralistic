#include "graphics-internal.hpp"

void gfx::RectArray::setRect(int index, RectShape rect) {
    vertex_array[index * 4].position = sf::Vector2f(rect.x, rect.y);
    vertex_array[index * 4 + 1].position = sf::Vector2f(rect.x + rect.w, rect.y);
    vertex_array[index * 4 + 2].position = sf::Vector2f(rect.x + rect.w, rect.y + rect.h);
    vertex_array[index * 4 + 3].position = sf::Vector2f(rect.x, rect.y + rect.h);
}

void gfx::RectArray::setColor(int index, Color color) {
    vertex_array[index].color = {color.r, color.g, color.b, color.a};
}

void gfx::RectArray::setTextureCoords(int index, RectShape texture_coordinates) {
    vertex_array[index * 4].texCoords = sf::Vector2f(texture_coordinates.x, texture_coordinates.y);
    vertex_array[index * 4 + 1].texCoords = sf::Vector2f(texture_coordinates.x + texture_coordinates.w, texture_coordinates.y);
    vertex_array[index * 4 + 2].texCoords = sf::Vector2f(texture_coordinates.x + texture_coordinates.w, texture_coordinates.y + texture_coordinates.h);
    vertex_array[index * 4 + 3].texCoords = sf::Vector2f(texture_coordinates.x, texture_coordinates.y + texture_coordinates.h);
}

gfx::RectArray::RectArray(int size) {
    resize(size);
}

void gfx::RectArray::resize(int size) {
    vertex_buffer.setPrimitiveType(sf::Quads);
    vertex_buffer.setUsage(sf::VertexBuffer::Stream);
    delete[] vertex_array;
    vertex_array = new sf::Vertex[size * 4];
}

gfx::RectArray::~RectArray() {
    delete[] vertex_array;
}

void gfx::RectArray::render(int size, const Texture* image) {
    const sf::Texture* texture = nullptr;
    if(image)
        texture = &image->sfml_render_texture->getTexture();
    
    sf::RenderStates states;
    states.texture = texture;
    
    vertex_buffer.create(size * 4);
    vertex_buffer.update(vertex_array);
    
    render_target->draw(vertex_buffer, states);
}
