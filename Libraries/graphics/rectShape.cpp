#include "graphics-internal.hpp"

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
