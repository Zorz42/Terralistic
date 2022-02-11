#include "graphics-internal.hpp"
#include "exception.hpp"

void gfx::RectArray::setRect(int index, RectShape rect) {
    if(index < 0 || index >= length)
        throw Exception("Rect index must be positive and in bounds.");
    
    vertex_array[index * 4].position = sf::Vector2f(rect.x, rect.y);
    vertex_array[index * 4 + 1].position = sf::Vector2f(rect.x + rect.w, rect.y);
    vertex_array[index * 4 + 2].position = sf::Vector2f(rect.x + rect.w, rect.y + rect.h);
    vertex_array[index * 4 + 3].position = sf::Vector2f(rect.x, rect.y + rect.h);
}

void gfx::RectArray::setColor(int index, Color color) {
    if(index < 0 || index >= length * 4)
        throw Exception("Color index must be positive and in bounds.");
    vertex_array[index].color = {color.r, color.g, color.b, color.a};
}

void gfx::RectArray::setTextureCoords(int index, RectShape texture_coordinates) {
    if(index < 0 || index >= length)
        throw Exception("Texture coord index must be positive and in bounds.");
    vertex_array[index * 4].texCoords = sf::Vector2f(texture_coordinates.x, texture_coordinates.y);
    vertex_array[index * 4 + 1].texCoords = sf::Vector2f(texture_coordinates.x + texture_coordinates.w, texture_coordinates.y);
    vertex_array[index * 4 + 2].texCoords = sf::Vector2f(texture_coordinates.x + texture_coordinates.w, texture_coordinates.y + texture_coordinates.h);
    vertex_array[index * 4 + 3].texCoords = sf::Vector2f(texture_coordinates.x, texture_coordinates.y + texture_coordinates.h);
}

gfx::RectArray::RectArray(int size) {
    if(size < 0)
        throw Exception("RectArray size must be positive.");
    resize(size);
}

void gfx::RectArray::resize(int size) {
    if(size < 0)
        throw Exception("RectArray size must be positive.");
    vertex_buffer.setPrimitiveType(sf::Quads);
    vertex_buffer.setUsage(sf::VertexBuffer::Static);
    delete[] vertex_array;
    vertex_array = new sf::Vertex[size * 4];
    length = size;
}

gfx::RectArray::~RectArray() {
    delete[] vertex_array;
}

void gfx::RectArray::render(int size, const Texture* image, int x, int y, bool blend_multiply) {
    if(size < 0)
        throw Exception("RectArray size must be positive.");
    
    const sf::Texture* texture = nullptr;
    if(image)
        texture = &image->sfml_render_texture->getTexture();
    
    sf::Transform transformation;
    transformation.translate(x, y);
    
    sf::RenderStates states;
    states.texture = texture;
    states.transform = transformation;
    
    if(blend_multiply)
        states.blendMode = sf::BlendMultiply;
    
    vertex_buffer.create(size * 4);
    vertex_buffer.update(vertex_array);
    
    render_target->draw(vertex_buffer, states);
}
