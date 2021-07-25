#include "graphics-internal.hpp"


gfx::Color::Color(unsigned char r, unsigned char g, unsigned char b, unsigned char a) : r(r), g(g), b(b), a(a) {}

gfx::RectShape::RectShape(short x, short y, unsigned short w, unsigned short h) : x(x), y(y), w(w), h(h) {}

void gfx::RectShape::render(Color c, bool fill) {
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
    return RectShape(getTranslatedX(), getTranslatedY(), getWidth(), getHeight());
}  
short gfx::_CenteredObject::getTranslatedX() const {
    return orientation % 3 == 1 ? (getWindowWidth() >> 1) - (getWidth() >> 1) + x : (orientation % 3 == 2 ? getWindowWidth() - getWidth() + x : x);
}

short gfx::_CenteredObject::getTranslatedY() const {
    return orientation / 3 == 1 ? (getWindowHeight() >> 1) - (getHeight() >> 1) + y : (orientation / 3 == 2 ? getWindowHeight() - getHeight() + y : y);
}

gfx::_CenteredObject::_CenteredObject(short x, short y, ObjectType orientation) : orientation(orientation), x(x), y(y) {}

gfx::Rect::Rect(short x, short y, unsigned short w, unsigned short h, Color c, ObjectType orientation) : _CenteredObject(x, y, orientation), w(w), h(h), c(c) {}

void gfx::Rect::render(bool fill) const {
    RectShape rect = getTranslatedRect();
    if(blur_intensity)
        blurRegion(rect, blur_intensity);
    rect.render(c, fill);
}
