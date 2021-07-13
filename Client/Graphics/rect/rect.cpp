#include "rect.hpp"
#include "../ui/ui.hpp"
#include "../graphics-internal.hpp"
//RectShape
RectShape::RectShape(short x, short y, unsigned short w, unsigned short h) : x(x), y(y), w(w), h(h) {}
void RectShape::render(Color c, bool fill) {
    SDL_SetRenderDrawColor(gfx::renderer, c.r, c.g, c.b, c.a);
        SDL_Rect sdl_rect = { this->x, this->y, this->w, this->h };
        if (fill)
            SDL_RenderFillRect(gfx::renderer, &sdl_rect);
        else
            SDL_RenderDrawRect(gfx::renderer, &sdl_rect);
    }
}


//_CenteredObject
RectShape _CenteredObject::getTranslatedRect() const {
    return rectShape(getTranslatedX(), getTranslatedY(), getWidth(), getHeight());
}  
short _CenteredObject::getTranslatedX() const {
    return orientation % 3 == 1 ? (window_width >> 1) - (getWidth() >> 1) + x : (orientation % 3 == 2 ? window_width - getWidth() + x : x);
}

short _CenteredObject::getTranslatedY() const {
    return orientation / 3 == 1 ? (window_height >> 1) - (getHeight() >> 1) + y : (orientation / 3 == 2 ? window_height - getHeight() + y : y);
}


_CenteredObject::_CenteredObject(short x, short y, objectType orientation) : orientation(orientation), x(x), y(y) {}

//Rect
explicit Rect::Rect(short x, short y, unsigned short w, unsigned short h, Color c, objectType orientation) : _centeredObject(x, y, orientation), w(w), h(h), c(c) {}
void Rect::render(bool fill) {
    SDL_SetRenderDrawColor(gfx::renderer, this->c.r, this->c.g, this->c.b, this->c.a);
    RectShape gfx_rect = this->getTranslatedRect();
    SDL_Rect sdl_rect = { gfx_rect.x, gfx_rect.y, gfx_rect.w, gfx_rect.h };
    if (fill)
        SDL_RenderFillRect(gfx::renderer, &sdl_rect);
    else
        SDL_RenderDrawRect(gfx::renderer, &sdl_rect);
}