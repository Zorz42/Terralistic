#include "graphics-internal.hpp"


gfx::Color::Color(unsigned char r, unsigned char g, unsigned char b, unsigned char a) : r(r), g(g), b(b), a(a) {}

gfx::RectShape::RectShape(short x, short y, unsigned short w, unsigned short h) : x(x), y(y), w(w), h(h) {}

void gfx::RectShape::render(Color c, bool fill) const {
    sf::RectangleShape rect(sf::Vector2f(w, h));
    rect.setPosition(x, y);
    if (fill) {
        rect.setOutlineColor(sf::Color::Transparent);
        rect.setFillColor((const sf::Color)c);
        render_target->draw(rect);
    } else {
        rect.setFillColor(sf::Color::Transparent);
        rect.setOutlineColor((const sf::Color)c);
        rect.setOutlineThickness(1);
        render_target->draw(rect);
    }
}


gfx::RectShape gfx::_CenteredObject::getTranslatedRect() const {
    return {getTranslatedX(), getTranslatedY(), getWidth(), getHeight()};
}  
short gfx::_CenteredObject::getTranslatedX() const {
    return orientation % 3 == 1 ? (getWindowWidth() >> 1) - (getWidth() >> 1) + x : (orientation % 3 == 2 ? getWindowWidth() - getWidth() + x : x);
}

short gfx::_CenteredObject::getTranslatedY() const {
    return orientation / 3 == 1 ? (getWindowHeight() >> 1) - (getHeight() >> 1) + y : (orientation / 3 == 2 ? getWindowHeight() - getHeight() + y : y);
}

gfx::_CenteredObject::_CenteredObject(short x, short y, ObjectType orientation) : orientation(orientation), x(x), y(y) {}

gfx::Rect::Rect(short x, short y, unsigned short w, unsigned short h, Color c, ObjectType orientation) : _CenteredObject(x, y, orientation), w(w), h(h), c(c), prev_x(x), prev_y(y), prev_w(w), prev_h(h) {}

void gfx::Rect::render(bool fill) {
    RectShape rect = getTranslatedRect();

    if(blur_intensity) {
        if(blur_intensity && !blur_texture)
            blur_texture = new sf::RenderTexture;
        if(blur_texture->getSize().x != w || blur_texture->getSize().y != h)
            blur_texture->create(w, h);
        
        blur_texture->clear({0, 0, 0});
        sf::Sprite back_sprite;
        back_sprite.setTexture(render_target->getTexture());
        back_sprite.setTextureRect({rect.x, rect.y, rect.w, rect.h});
        blur_texture->draw(back_sprite);
        blur_texture->display();
        
        blurTexture(*blur_texture, blur_intensity);
        blur_texture->display();
        
        sf::Sprite sprite;
        sprite.setTexture(blur_texture->getTexture());
        sprite.setPosition(getTranslatedX(), getTranslatedY());
        render_target->draw(sprite);
    }
    
    if(shadow_intensity) {
        if(!shadow_texture) {
            shadow_texture = new sf::RenderTexture;
            updateShadowTexture();
        }
        if(prev_x != x || prev_y != y || prev_w != w || prev_h != h) {
            prev_x = x;
            prev_y = y;
            prev_w = w;
            prev_h = h;
            updateShadowTexture();
        }
        if(getWindowWidth() != shadow_texture->getSize().x || getWindowHeight() != shadow_texture->getSize().y)
            updateShadowTexture();
        
        render_target->draw(sf::Sprite(shadow_texture->getTexture()));
    }
    rect.render(c, fill);
}

void gfx::Rect::updateShadowTexture() {
    if(getWindowWidth() != shadow_texture->getSize().x || getWindowHeight() != shadow_texture->getSize().y)
        shadow_texture->create(getWindowWidth(), getWindowHeight());
    shadow_texture->clear({0, 0, 0, 0});
    
    sf::RectangleShape rect(sf::Vector2f(getWidth(), getHeight()));
    rect.setPosition(getTranslatedX(), getTranslatedY());
    rect.setFillColor({0, 0, 0, shadow_intensity});
    shadow_texture->draw(rect);
    
    shadow_texture->display();
    
    blurTexture(*shadow_texture, shadow_blur, 2);
    
    rect.setFillColor({0, 0, 0, 0});
    shadow_texture->draw(rect, sf::BlendNone);
    
    shadow_texture->display();
}

gfx::Rect::~Rect() {
    delete shadow_texture;
    delete blur_texture;
}
