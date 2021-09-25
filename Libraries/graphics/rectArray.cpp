#include "graphics-internal.hpp"

void gfx::RectArray::setRect(unsigned short index, RectShape rect) {
    vertex_array[index * 4].position = sf::Vector2f(rect.x, rect.y);
    vertex_array[index * 4 + 1].position = sf::Vector2f(rect.x + rect.w, rect.y);
    vertex_array[index * 4 + 2].position = sf::Vector2f(rect.x + rect.w, rect.y + rect.h);
    vertex_array[index * 4 + 3].position = sf::Vector2f(rect.x, rect.y + rect.h);
}

void gfx::RectArray::setColor(unsigned short index, Color color) {
    vertex_array[index].color = {color.r, color.g, color.b, color.a};
}

void gfx::RectArray::setTextureCoords(unsigned short index, RectShape texture_coordinates) {
    vertex_array[index * 4].texCoords = sf::Vector2f(texture_coordinates.x, texture_coordinates.y);
    vertex_array[index * 4 + 1].texCoords = sf::Vector2f(texture_coordinates.x + texture_coordinates.w, texture_coordinates.y);
    vertex_array[index * 4 + 2].texCoords = sf::Vector2f(texture_coordinates.x + texture_coordinates.w, texture_coordinates.y + texture_coordinates.h);
    vertex_array[index * 4 + 3].texCoords = sf::Vector2f(texture_coordinates.x, texture_coordinates.y + texture_coordinates.h);
}

void gfx::RectArray::setImage(const Image* image_) {
    image = image_;
}

void gfx::RectArray::render() {
    const sf::Texture* texture = nullptr;
    if(image)
        texture = &image->getSfmlTexture()->getTexture();
    
    sf::RenderStates states;
    states.texture = texture;
    render_target->draw(vertex_array, states);
}

void gfx::RectArray::resize(unsigned short size) {
    vertex_array.resize(size * 4);
}
