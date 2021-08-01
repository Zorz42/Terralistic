#include "graphics-internal.hpp"

void gfx::Image::createBlankImage(unsigned short width, unsigned short height) {
    delete sfml_render_texture;
    sfml_render_texture = new sf::RenderTexture;
    bool result = sfml_render_texture->create(width, height);
    assert(result);
    clear();
}

void gfx::Image::renderText(const std::string& text, Color text_color) {
    sf::Text sf_text;
    sf_text.setFont(font);
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
    bool result = image_texture.loadFromFile(resource_path + path);
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
    if(free_texture) {
        delete sfml_render_texture;
        sfml_render_texture = nullptr;
    }
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
    sfml_render_texture->display();
    sf::Sprite sprite;
    sprite.setTexture(sfml_render_texture->getTexture());
    sprite.setTextureRect(sf::IntRect(src_rect.x, src_rect.y, src_rect.w, src_rect.h));
    int x_offset = flipped ? getTextureWidth() : 0;
    sprite.setPosition(x + x_offset * scale, y);
    int flip_factor = flipped ? -1 : 1;
    sprite.setScale(flip_factor * scale, scale);
    sprite.setColor({color.r, color.g, color.b, color.a});
    render_target->draw(sprite);
}

void gfx::Image::render(float scale, short x, short y, bool flipped) const {
    render(scale, x, y, {0, 0, getTextureWidth(), getTextureHeight()}, flipped);
}

gfx::Sprite::Sprite() : _CenteredObject(0, 0) {}

void gfx::Sprite::render() const {
    Image::render(scale, getTranslatedX(), getTranslatedY(), flipped);
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
    RectShape rect = getTranslatedRect();
    int hover_progress_target = isHovered() ? 255 : 0;
    hover_progress += (hover_progress_target - (int)hover_progress) / 2;
    Color button_color{
        (unsigned char)((int)hover_color.r * (int)hover_progress / 255 + (int)def_color.r * (int)(255 - hover_progress) / 255),
        (unsigned char)((int)hover_color.g * (int)hover_progress / 255 + (int)def_color.g * (int)(255 - hover_progress) / 255),
        (unsigned char)((int)hover_color.b * (int)hover_progress / 255 + (int)def_color.b * (int)(255 - hover_progress) / 255),
        (unsigned char)((int)hover_color.a * (int)hover_progress / 255 + (int)def_color.a * (int)(255 - hover_progress) / 255),
    };
    rect.render(button_color);
    float ms = margin * scale;
    Image::render(scale, rect.x + ms, rect.y + ms);
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
    back_rect.shadow_intensity = GFX_DEFAULT_TEXT_BOX_SHADOW_INTENSITY;
    temp.renderText("|g", { 0, 0, 0 });
    cut_length = temp.getTextureWidth() - 1;
}

void gfx::TextInput::setBlurIntensity(float blur_intensity) {
    back_rect.blur_intensity = blur_intensity;
}

void gfx::TextInput::render() {
    RectShape rect = getTranslatedRect();
    back_rect.x = rect.x;
    back_rect.y = rect.y;
    back_rect.w = rect.w;
    back_rect.h = rect.h;
    back_rect.c = isHovered() ? hover_color : def_color;
    back_rect.render();
    
    rect.x += margin * scale;
    rect.y += margin * scale;
    rect.w = getTextureWidth() * scale;
    rect.h -= margin * 2 * scale;
    short x;
    unsigned short w;
    if (rect.w - cut_length > width * scale) {
        x = rect.w / scale - width;
        w = width;
    }
    else {
        x = cut_length;
        w = rect.w / scale - cut_length;
    }
    
    Image::render(scale, rect.x, rect.y, {x, 0, w, (unsigned short)(rect.h / scale)});
    if (active) {
        Rect rec(rect.x + (rect.w > width * scale ? width * scale : rect.w - cut_length * scale), rect.y, scale, rect.h, text_color);
        rec.render();
    }
        
}

void gfx::TextInput::setBorderColor(Color color) {
    back_rect.border_color = color;
}
