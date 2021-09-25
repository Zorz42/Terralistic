#include "graphics-internal.hpp"
#include <cassert>

void gfx::Image::createBlankImage(unsigned short width, unsigned short height) {
    freeTexture();
    sfml_render_texture = new sf::RenderTexture;
    bool result = sfml_render_texture->create(width, height);
    assert(result);
    clear();
}

void gfx::Image::loadFromText(const std::string& text, Color text_color) {
    sf::Text sf_text;
    sf_text.setFont(font);
    sf_text.setString("|g");
    sf_text.setFillColor({text_color.r, text_color.g, text_color.b, text_color.a});
    sf_text.setCharacterSize(font_size);
    int width_to_cut = sf_text.getLocalBounds().width;
    sf_text.setString(std::string("|g") + text);
    
    sf::Rect text_rect = sf_text.getLocalBounds();
    sf_text.setOrigin(text_rect.left + text_rect.width / 2, text_rect.top  + text_rect.height / 2);
    sf_text.setPosition(sf::Vector2f(sf_text.getLocalBounds().width / 2 - width_to_cut, sf_text.getLocalBounds().height / 2));
    
    int width = sf_text.getLocalBounds().width - width_to_cut;
    if(!width)
        width = 1;
    createBlankImage(width, sf_text.getLocalBounds().height);
    sfml_render_texture->draw(sf_text);
    sfml_render_texture->display();
}

void gfx::Image::loadFromResources(const std::string& path) {
    loadFromFile(resource_path + path);
}

void gfx::Image::loadFromFile(const std::string& path) {
    sf::Texture image_texture;
    bool result = image_texture.loadFromFile(path);
    assert(result);
    sf::RectangleShape sfml_rect;
    sf::Vector2u size = image_texture.getSize();
    sfml_rect.setSize({(float)size.x, (float)size.y});
    sfml_rect.setTexture(&image_texture);
    
    createBlankImage(size.x, size.y);
    sfml_render_texture->draw(sfml_rect);
    sfml_render_texture->display();
}

gfx::Image::~Image() {
    freeTexture();
}

void gfx::Image::freeTexture() {
    delete sfml_render_texture;
    sfml_render_texture = nullptr;
}

void gfx::Image::setColor(Color color_) {
    color = color_;
}

unsigned short gfx::Image::getTextureWidth() const {
    return sfml_render_texture->getSize().x;
}

unsigned short gfx::Image::getTextureHeight() const {
    return sfml_render_texture->getSize().y;
}

void gfx::Image::clear() {
    sfml_render_texture->clear({0, 0, 0, 0});
}
void gfx::Image::render(float scale, short x, short y, RectShape src_rect, bool flipped) const {
    sf::Sprite sprite;
    sprite.setTexture(sfml_render_texture->getTexture());
    sprite.setTextureRect({src_rect.x, src_rect.y, src_rect.w, src_rect.h});
    int x_offset = flipped ? src_rect.w : 0;
    sprite.setPosition(x + x_offset * scale, y);
    int flip_factor = flipped ? -1 : 1;
    sprite.setScale(flip_factor * scale, scale);
    sprite.setColor({color.r, color.g, color.b, color.a});
    render_target->draw(sprite);
}

void gfx::Image::render(float scale, short x, short y, bool flipped) const {
    render(scale, x, y, {0, 0, getTextureWidth(), getTextureHeight()}, flipped);
}

void gfx::Image::loadFromPixelGrid(const PixelGrid& pixel_grid) {
    sf::Texture texture;
    texture.create(pixel_grid.getWidth(), pixel_grid.getHeight());
    texture.update(pixel_grid.getArray());
    
    createBlankImage(pixel_grid.getWidth(), pixel_grid.getHeight());
    sfml_render_texture->draw(sf::Sprite(texture));
    sfml_render_texture->display();
}
