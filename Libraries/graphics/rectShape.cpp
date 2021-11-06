#include "graphics-internal.hpp"

gfx::RectShape::RectShape(int x, int y, int w, int h) : x(x), y(y), w(w), h(h) {}

void gfx::RectShape::render(Color color) const {
    sf::RectangleShape rect;
    rect.setSize({(float)w, (float)h});
    rect.setPosition(x, y);
    rect.setFillColor({color.r, color.g, color.b, color.a});
    render_target->draw(rect);
}

void gfx::RectShape::renderOutline(Color color) const {
    sf::RectangleShape rect;
    rect.setSize({(float)w, (float)h});
    rect.setPosition(x, y);
    rect.setFillColor(sf::Color::Transparent);
    rect.setOutlineColor({color.r, color.g, color.b, color.a});
    rect.setOutlineThickness(1);
    render_target->draw(rect);
}

