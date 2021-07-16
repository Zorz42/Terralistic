#include "graphics-internal.hpp"

gfx::RectShape::RectShape(short x, short y, unsigned short w, unsigned short h) : x(x), y(y), w(w), h(h) {}

void gfx::RectShape::render(Color c, bool fill) {
    SDL_SetRenderDrawColor(gfx::renderer, c.r, c.g, c.b, c.a);
    SDL_Rect sdl_rect = { this->x, this->y, this->w, this->h };
    if (fill)
        SDL_RenderFillRect(gfx::renderer, &sdl_rect);
    else
        SDL_RenderDrawRect(gfx::renderer, &sdl_rect);
}


gfx::RectShape gfx::_CenteredObject::getTranslatedRect() const {
    return RectShape(getTranslatedX(), getTranslatedY(), getWidth(), getHeight());
}  
short gfx::_CenteredObject::getTranslatedX() const {
    return orientation % 3 == 1 ? (window_width >> 1) - (getWidth() >> 1) + x : (orientation % 3 == 2 ? window_width - getWidth() + x : x);
}

short gfx::_CenteredObject::getTranslatedY() const {
    return orientation / 3 == 1 ? (window_height >> 1) - (getHeight() >> 1) + y : (orientation / 3 == 2 ? window_height - getHeight() + y : y);
}


gfx::_CenteredObject::_CenteredObject(short x, short y, ObjectType orientation) : orientation(orientation), x(x), y(y) {}

gfx::Rect::Rect(short x, short y, unsigned short w, unsigned short h, Color c, ObjectType orientation) : _CenteredObject(x, y, orientation), w(w), h(h), c(c) {}

void gfx::Rect::render(bool fill) const {
    SDL_SetRenderDrawColor(gfx::renderer, this->c.r, this->c.g, this->c.b, this->c.a);
    RectShape gfx_rect = this->getTranslatedRect();
    SDL_Rect sdl_rect = { gfx_rect.x, gfx_rect.y, gfx_rect.w, gfx_rect.h };
    if (fill)
        SDL_RenderFillRect(gfx::renderer, &sdl_rect);
    else
        SDL_RenderDrawRect(gfx::renderer, &sdl_rect);
}


gfx::Color::Color(unsigned char r, unsigned char g, unsigned char b, unsigned char a) : r(r), g(g), b(b), a(a) {}
