#include "glfwAbstraction.hpp"
#include "container.hpp"

gfx::Container::Container(int x, int y, int w, int h, Orientation orientation) : orientation(orientation), x(x), y(y), w(w), h(h) {}

gfx::RectShape gfx::Container::getTranslatedRect() const {
    RectShape parent_rect = parent_containter == nullptr ? RectShape{0, 0, getWindowWidth(), getWindowHeight()} : parent_containter->getTranslatedRect();
    
    int translated_x = parent_rect.x + x + parent_rect.w * orientation.x - w * orientation.x;
    int translated_y = parent_rect.y + y + parent_rect.h * orientation.y - h * orientation.y;
    
    return {translated_x, translated_y, w, h};
}
