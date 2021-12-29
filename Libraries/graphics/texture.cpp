#include "graphics-internal.hpp"
#include "exception.hpp"

void gfx::Texture::createBlankImage(int width, int height) {
    if(width <= 0 || height <= 0)
        throw Exception("Width and Height of a texture size must be positive.");
    
    freeTexture();
    sfml_render_texture = new sf::RenderTexture;
    if(!sfml_render_texture->create(width, height))
        throw Exception("Could not create texture.");
    clear();
}

void gfx::Texture::loadFromText(const std::string& text, Color text_color) {
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

void gfx::Texture::loadFromResources(const std::string& path) {
    loadFromFile(resource_path + path);
}

void gfx::Texture::loadFromFile(const std::string& path) {
    sf::Texture image_texture;
    if(!image_texture.loadFromFile(path))
        throw Exception("Could not load file " + path);
    
    sf::RectangleShape sfml_rect;
    sf::Vector2u size = image_texture.getSize();
    sfml_rect.setSize({(float)size.x, (float)size.y});
    sfml_rect.setTexture(&image_texture);
    
    createBlankImage(size.x, size.y);
    sfml_render_texture->draw(sfml_rect);
    sfml_render_texture->display();
}

gfx::Texture::~Texture() {
    freeTexture();
}

void gfx::Texture::freeTexture() {
    delete sfml_render_texture;
    sfml_render_texture = nullptr;
}

void gfx::Texture::setColor(Color color_) {
    color = color_;
}

int gfx::Texture::getTextureWidth() const {
    return sfml_render_texture->getSize().x;
}

int gfx::Texture::getTextureHeight() const {
    return sfml_render_texture->getSize().y;
}

void gfx::Texture::clear() {
    sfml_render_texture->clear({0, 0, 0, 0});
}

void gfx::Texture::render(float scale, int x, int y, RectShape src_rect, bool flipped) const {
    if(scale <= 0)
        throw Exception("Texture scale must be positive.");
    
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

void gfx::Texture::render(float scale, int x, int y, bool flipped) const {
    render(scale, x, y, {0, 0, getTextureWidth(), getTextureHeight()}, flipped);
}
