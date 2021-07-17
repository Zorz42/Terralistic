#include "graphics-internal.hpp"

void gfx::Image::createBlankImage(unsigned short width, unsigned short height) {
    type = ImageType::RENDER_TEXTURE;
    delete sfml_render_texture;
    sfml_render_texture = new sf::RenderTexture;
    assert(sfml_render_texture->create(width, height));
}

void gfx::Image::renderText(const std::string& text, Color text_color) {
    type = ImageType::TEXT;
    delete sfml_text;
    sfml_text = new sf::Text();
    sfml_text->setFont(sfml_font);
    sfml_text->setString(text.c_str());
    sfml_text->setFillColor((sf::Color)text_color);
}

void gfx::Image::loadFromFile(const std::string& path) {
    type = ImageType::TEXTURE;
    assert(sfml_texture.loadFromFile((resource_path + path).c_str()));
}

gfx::Image::~Image() {
    freeTexture();
}

void gfx::Image::freeTexture() {
    if(texture && free_texture) {
        delete sfml_render_texture;
        sfml_render_texture = nullptr;
    }
}

unsigned short gfx::Image::getTextureWidth() const {
    if(type == ImageType::RENDER_TEXTURE)
        return sfml_render_texture->getSize().x;
    else if(type == ImageType::TEXTURE)
        return sfml_texture.getSize().x;
    else
        return sfml_text->getLocalBounds().width;
}

unsigned short gfx::Image::getTextureHeight() const {
    if(type == ImageType::RENDER_TEXTURE)
        return sfml_render_texture->getSize().y;
    else if(type == ImageType::TEXTURE)
        return sfml_texture.getSize().y;
    else
        return sfml_text->getLocalBounds().height;
}

void gfx::Image::clear() {
    sfml_render_texture->clear({0, 0, 0, 0});
}
void gfx::Image::render(float scale, short x, short y, RectShape src_rect) const {

    //sfml_text.setCharacterSize(scale);
    if(type == ImageType::TEXT) {
        sfml_text->setPosition((float)x, (float)y);
        render_target->draw(*sfml_text);
    } else if(type == ImageType::TEXTURE) {
        sf::Sprite sprite;
        sprite.setTexture(sfml_texture);
        sprite.setTextureRect(sf::IntRect(src_rect.x, src_rect.y, src_rect.w, src_rect.h));
        sprite.setPosition(x, y);
        sprite.setScale(scale, scale);
        render_target->draw(sprite);
    } else if(type == ImageType::RENDER_TEXTURE) {
        sfml_render_texture->display();
        sf::Sprite sprite;
        sprite.setTexture(sfml_render_texture->getTexture());
        sprite.setTextureRect(sf::IntRect(src_rect.x, src_rect.y, src_rect.w, src_rect.h));
        sprite.setPosition(x, y);
        sprite.setScale(scale, scale);
        render_target->draw(sprite);
    }
}

void gfx::Image::setAlpha(unsigned char alpha) {
    // TODO: set alpha
}

void gfx::Image::render(float scale, short x, short y) const {
    if (type == ImageType::TEXT) {
        sfml_text->setPosition(sf::Vector2f(x, y));
        sfml_text->setCharacterSize(scale * font_size);
        render_target->draw(*sfml_text);
    } else if(type == ImageType::TEXTURE) {
        sf::RectangleShape sfml_rect;
        sfml_rect.setPosition(x, y);
        sfml_rect.setSize({(float)getTextureWidth(), (float)getTextureHeight()});
        sfml_rect.setScale(scale, scale);
        sfml_rect.setTexture(&sfml_texture);
        render_target->draw(sfml_rect);
    } else if(type == ImageType::RENDER_TEXTURE) {
        sf::RectangleShape sfml_rect;
        sfml_rect.setPosition(x, y);
        sfml_rect.setSize({(float)getTextureWidth(), (float)getTextureHeight()});
        sfml_rect.setScale(scale, scale);
        sfml_rect.setTexture(&sfml_render_texture->getTexture());
        render_target->draw(sfml_rect);
    }
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
