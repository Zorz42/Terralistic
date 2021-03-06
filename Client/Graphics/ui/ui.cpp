#include "graphics-internal.hpp"

void gfx::Image::createBlankImage(unsigned short width, unsigned short height) {
    delete sfml_render_texture;
    sfml_render_texture = new sf::RenderTexture;
    assert(sfml_render_texture->create(width, height));
    clear();
}

void gfx::Image::renderText(const std::string& text, Color text_color) {
    sf::Text sf_text;
    sf_text.setFont(sfml_font);
    sf_text.setString(text.c_str());
    sf_text.setFillColor((sf::Color)text_color);
    sf_text.setCharacterSize(font_size);
    
    sf::FloatRect text_rect = sf_text.getLocalBounds();
    sf_text.setOrigin(text_rect.left + text_rect.width / 2, text_rect.top  + text_rect.height / 2);
    sf_text.setPosition(sf::Vector2f(sf_text.getLocalBounds().width / 2, sf_text.getLocalBounds().height / 2));
    
    createBlankImage(sf_text.getLocalBounds().width, sf_text.getLocalBounds().height);
    sfml_render_texture->draw(sf_text);
    sfml_render_texture->display();
}

void gfx::Image::loadFromFile(const std::string& path) {
    sf::Texture image_texture;
    assert(image_texture.loadFromFile(resource_path + path));
    sf::RectangleShape sfml_rect;
    sfml_rect.setSize({(float)image_texture.getSize().x, (float)image_texture.getSize().y});
    sfml_rect.setTexture(&image_texture);
    
    createBlankImage(image_texture.getSize().x, image_texture.getSize().y);
    sfml_render_texture->draw(sfml_rect);
    sfml_render_texture->display();
}

gfx::Image::~Image() {
    freeTexture();
}

void gfx::Image::freeTexture() {
    if(free_texture) {
        delete sfml_render_texture;
        sfml_render_texture = nullptr;
    }
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
void gfx::Image::render(float scale, short x, short y, RectShape src_rect) const {
    sfml_render_texture->display();
    sf::Sprite sprite;
    sprite.setTexture(sfml_render_texture->getTexture());
    sprite.setTextureRect(sf::IntRect(src_rect.x, src_rect.y, src_rect.w, src_rect.h));
    sprite.setPosition(x, y);
    sprite.setScale(scale, scale);
    render_target->draw(sprite);
}

void gfx::Image::setAlpha(unsigned char alpha) {
    // TODO: set alpha
}

void gfx::Image::render(float scale, short x, short y) const {
    sfml_render_texture->display();
    sf::RectangleShape sfml_rect;
    sfml_rect.setPosition(x, y);
    sfml_rect.setSize({(float)getTextureWidth(), (float)getTextureHeight()});
    sfml_rect.setScale(scale, scale);
    sfml_rect.setTexture(&sfml_render_texture->getTexture());
    render_target->draw(sfml_rect);
}

gfx::Sprite::Sprite() : _CenteredObject(0, 0) {};

void gfx::Sprite::render() const {
    Image::render(scale, getTranslatedX(), getTranslatedY());
}

unsigned short gfx::Button::getWidth() const {
    return (getTextureWidth() + (margin << 1)) * scale;
}

unsigned short gfx::Button::getHeight() const {
    return (getTextureHeight() + (margin << 1)) * scale;
}

bool gfx::Button::isHovered() const {
    if (disabled)
        return false;
    RectShape rect = getTranslatedRect();
    return mouse_x >= rect.x && mouse_y >= rect.y && mouse_x <= rect.x + rect.w && mouse_y <= rect.y + rect.h;
}

void gfx::Button::render() {
    RectShape rect = this->getTranslatedRect();
    int hover_progress_target = this->isHovered() ? 255 : 0;
    this->hover_progress += (hover_progress_target - (int)this->hover_progress) / 2;
    Color button_color{
        (unsigned char)((int)this->hover_color.r * (int)this->hover_progress / 255 + (int)this->def_color.r * (int)(255 - this->hover_progress) / 255),
        (unsigned char)((int)this->hover_color.g * (int)this->hover_progress / 255 + (int)this->def_color.g * (int)(255 - this->hover_progress) / 255),
        (unsigned char)((int)this->hover_color.b * (int)this->hover_progress / 255 + (int)this->def_color.b * (int)(255 - this->hover_progress) / 255),
    };
    rect.render(button_color);
    Image::render(scale, rect.x + margin * scale, rect.y + margin * scale);
}

void gfx::TextInput::setText(const std::string& text_) {
    text = text_;
    renderText((std::string)"|g" + text, text_color);
}

unsigned short gfx::TextInput::getWidth() const {
    return (width + 2 * margin) * scale;
}

gfx::TextInput::TextInput() {
    margin = 3;
    Image temp;
    temp.renderText("|g", { 0, 0, 0 });
    cut_length = temp.getTextureWidth() - 1;
}

void gfx::TextInput::render() const {
    RectShape rect = this->getTranslatedRect();
    rect.render(border_color);
    RectShape(rect.x + this->scale, rect.y + this->scale, rect.w - this->scale * 2, rect.h - this->scale * 2).render(isHovered() ? hover_color : def_color);
    rect.x += this->margin * this->scale;
    rect.y += this->margin * this->scale;
    rect.w = this->getTextureWidth() * this->scale;
    rect.h -= this->margin * 2 * this->scale;
    Image::render(scale, rect.x, rect.y, RectShape(rect.w - this->cut_length > this->width * this->scale ? rect.w / this->scale - this->width : this->cut_length, 0, rect.w - this->cut_length > this->width * this->scale ? this->width : rect.w / this->scale - this->cut_length, rect.h / this->scale));
    if (active)
        Rect(rect.x + (rect.w > width * scale ? width * scale : rect.w - cut_length * scale), rect.y, scale, rect.h, text_color).render();
}
