#include "graphics-internal.hpp"

gfx::RectShape::RectShape(short x, short y, unsigned short w, unsigned short h) : x(x), y(y), w(w), h(h) {}

void gfx::RectShape::render(Color c) const {
    sf::RectangleShape rect;
    rect.setSize({(float)w, (float)h});
    rect.setPosition(x, y);
    rect.setFillColor((const sf::Color)c);
    render_target->draw(rect);
}

void gfx::RectShape::renderOutline(Color c) const {
    sf::RectangleShape rect;
    rect.setSize({(float)w, (float)h});
    rect.setPosition(x, y);
    rect.setFillColor(sf::Color::Transparent);
    rect.setOutlineColor((const sf::Color)c);
    rect.setOutlineThickness(1);
    render_target->draw(rect);
}

